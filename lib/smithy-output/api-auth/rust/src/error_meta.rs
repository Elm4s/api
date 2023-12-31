// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
	/// An error thrown when the requestee has sent an invalid or malformed request.
	BadRequestError(crate::error::BadRequestError),
	/// An error thrown when the requestee requests a resource they do not have access to.
	ForbiddenError(crate::error::ForbiddenError),
	/// An error caused by internal server problems.
	InternalError(crate::error::InternalError),
	/// An error thrown when the requestee requests a non existant resource.
	NotFoundError(crate::error::NotFoundError),
	/// An error thrown when the requestee has hit a rate limit. You are sending too many requests too quickly.
	RateLimitError(crate::error::RateLimitError),
	/// An error thrown when the requestee is not authenticated.
	UnauthorizedError(crate::error::UnauthorizedError),
	/// An unhandled error occurred.
	Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::BadRequestError(inner) => inner.fmt(f),
			Error::ForbiddenError(inner) => inner.fmt(f),
			Error::InternalError(inner) => inner.fmt(f),
			Error::NotFoundError(inner) => inner.fmt(f),
			Error::RateLimitError(inner) => inner.fmt(f),
			Error::UnauthorizedError(inner) => inner.fmt(f),
			Error::Unhandled(inner) => inner.fmt(f),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CompleteEmailVerificationError, R>>
	for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(
		err: aws_smithy_http::result::SdkError<crate::error::CompleteEmailVerificationError, R>,
	) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::CompleteEmailVerificationErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::CompleteEmailVerificationErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::CompleteEmailVerificationErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::CompleteEmailVerificationErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::CompleteEmailVerificationErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::CompleteEmailVerificationErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::CompleteEmailVerificationErrorKind::Unhandled(inner) => {
					Error::Unhandled(inner)
				}
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RefreshIdentityTokenError, R>>
	for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(
		err: aws_smithy_http::result::SdkError<crate::error::RefreshIdentityTokenError, R>,
	) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::RefreshIdentityTokenErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::RefreshIdentityTokenErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::RefreshIdentityTokenErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::RefreshIdentityTokenErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::RefreshIdentityTokenErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::RefreshIdentityTokenErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::RefreshIdentityTokenErrorKind::Unhandled(inner) => {
					Error::Unhandled(inner)
				}
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartEmailVerificationError, R>>
	for Error
where
	R: Send + Sync + std::fmt::Debug + 'static,
{
	fn from(
		err: aws_smithy_http::result::SdkError<crate::error::StartEmailVerificationError, R>,
	) -> Self {
		match err {
			aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
				crate::error::StartEmailVerificationErrorKind::InternalError(inner) => {
					Error::InternalError(inner)
				}
				crate::error::StartEmailVerificationErrorKind::RateLimitError(inner) => {
					Error::RateLimitError(inner)
				}
				crate::error::StartEmailVerificationErrorKind::ForbiddenError(inner) => {
					Error::ForbiddenError(inner)
				}
				crate::error::StartEmailVerificationErrorKind::UnauthorizedError(inner) => {
					Error::UnauthorizedError(inner)
				}
				crate::error::StartEmailVerificationErrorKind::NotFoundError(inner) => {
					Error::NotFoundError(inner)
				}
				crate::error::StartEmailVerificationErrorKind::BadRequestError(inner) => {
					Error::BadRequestError(inner)
				}
				crate::error::StartEmailVerificationErrorKind::Unhandled(inner) => {
					Error::Unhandled(inner)
				}
			},
			_ => Error::Unhandled(err.into()),
		}
	}
}
impl std::error::Error for Error {}
