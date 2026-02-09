use std::{fmt::Display, str::FromStr, sync::Arc};

use crate::{
    adapters::crypto::errors::{CryptoError, CryptoResult},
    application::{AppResult, repositories::hash::Hasher},
    utils::{
        b64::{b64_decode, b64_encode},
        time::{now_utc_plus_sec_str, parse_utc, utc_now},
    },
};

#[derive(Debug)]
pub struct Token {
    pub ident: String,
    pub exp: String,
    pub sign_b64u: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            b64_encode(&self.ident),
            b64_encode(&self.exp),
            &self.sign_b64u,
        )
    }
}

impl FromStr for Token {
    type Err = CryptoError;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let splits = s.split(".").collect::<Vec<&str>>();

        if splits.len() != 3 {
            return Err(CryptoError::TokenInvalid);
        }

        let (ident, exp, sign_b64u) = (splits[0], splits[1], splits[2]);

        Ok(Self {
            ident: b64_decode(ident).map_err(|_| CryptoError::TokenCannotDecodeIdent)?,
            exp: b64_decode(exp).map_err(|_| CryptoError::TokenCannotDecodeExp)?,
            sign_b64u: sign_b64u.to_string(),
        })
    }
}

pub async fn generate_token(hasher: Arc<dyn Hasher>, ident: &str) -> AppResult<Token> {
    let exp = now_utc_plus_sec_str(1800);

    let sign = hasher
        .hash(format!("{}.{}", b64_encode(ident), b64_encode(&exp)))
        .await?;

    let sign_b64u = b64_encode(&sign);

    Ok(Token {
        ident: ident.to_string(),
        exp: exp,
        sign_b64u: sign_b64u,
    })
}

pub async fn validate_token(hasher: Arc<dyn Hasher>, origin_token: &Token) -> CryptoResult<()> {
    let ident_and_exp_b64u = format!(
        "{}.{}",
        b64_encode(&origin_token.ident),
        b64_encode(&origin_token.exp)
    );

    let sign =
        b64_decode(&origin_token.sign_b64u).map_err(|_| CryptoError::TokenCannotDecodeSign)?;

    hasher
        .validate(ident_and_exp_b64u, sign)
        .await
        .map_err(|_| CryptoError::TokenInvalid)?;

    let origin_exp = parse_utc(&origin_token.exp).map_err(|_| CryptoError::TokenExpNotIso)?;

    let now = utc_now();

    println!("   ->> token validation - now: {now:?} - origin_exp: {origin_exp:?}");

    if origin_exp < now {
        return Err(CryptoError::TokenExpired);
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("Can not match tokens")]
    TokenNotMatch,

    #[error("error")]
    TokenExpired,

    #[error("error")]
    TokenExpNotIso,

    #[error("error")]
    TokenCannotDecode,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::adapters::crypto::argon::ArgonHasher;

    use super::*;
    use anyhow::Result;
    use tokio::time::sleep;

    #[test]
    fn test_token() -> Result<()> {
        let fx_token = Token {
            ident: "fx-user".to_string(),
            exp: "2026-05-17T15:30:00Z".to_string(),
            sign_b64u: "sign".to_string(),
        };

        println!("-->> {}", fx_token);

        Ok(())
    }

    #[test]
    fn test_token_from_str_ok() -> anyhow::Result<()> {
        let fx_token_str = "ZngtdXNlcg.MjAyNi0wNS0xN1QxNTozMDowMFo.sign";

        let fx_token = Token {
            ident: "fx-user".to_string(),
            exp: "2026-05-17T15:30:00Z".to_string(),
            sign_b64u: "sign".to_string(),
        };

        let token = fx_token_str.parse::<Token>()?;

        assert_eq!(format!("{:?}", token), format!("{:?}", fx_token));

        Ok(())
    }

    #[tokio::test]
    async fn test_token_validation() -> anyhow::Result<()> {
        let hasher = Arc::new(ArgonHasher::new());

        let ident = "fx_user";

        let fx_token = generate_token(hasher.clone(), ident).await?;

        sleep(Duration::from_millis(10)).await;

        validate_token(hasher, &fx_token).await?;

        Ok(())
    }
}
