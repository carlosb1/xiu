use bytes::BytesMut;
use failure::Fail;

use libflv::errors::FlvDemuxerError;
use rtmp::session::errors::SessionError;

use networkio::bytes_errors::BytesWriteError;
use rtmp::amf0::errors::Amf0WriteError;
use rtmp::cache::errors::MetadataError;
use std::fmt;
// use tokio::sync::mpsc::error::SendError;

#[derive(Debug)]
pub struct ServerError {
    pub value: ServerErrorValue,
}

#[derive(Debug, Fail)]
pub enum ServerErrorValue {
    #[fail(display = "server error")]
    Error,
}
#[derive(Debug)]
pub struct MediaError {
    pub value: MediaErrorValue,
}

#[derive(Debug, Fail)]
pub enum MediaErrorValue {
    #[fail(display = "server error")]
    Error,
    #[fail(display = "session error")]
    SessionError(SessionError),

    #[fail(display = "amf write error")]
    Amf0WriteError(Amf0WriteError),
    #[fail(display = "metadata error")]
    MetadataError(MetadataError),
    #[fail(display = "flv demuxer error")]
    FlvDemuxerError(FlvDemuxerError),
}

impl From<SessionError> for MediaError {
    fn from(error: SessionError) -> Self {
        MediaError {
            value: MediaErrorValue::SessionError(error),
        }
    }
}

impl From<FlvDemuxerError> for MediaError {
    fn from(error: FlvDemuxerError) -> Self {
        MediaError {
            value: MediaErrorValue::FlvDemuxerError(error),
        }
    }
}

impl From<Amf0WriteError> for MediaError {
    fn from(error: Amf0WriteError) -> Self {
        MediaError {
            value: MediaErrorValue::Amf0WriteError(error),
        }
    }
}

impl From<MetadataError> for MediaError {
    fn from(error: MetadataError) -> Self {
        MediaError {
            value: MediaErrorValue::MetadataError(error),
        }
    }
}

impl fmt::Display for MediaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}

pub struct HlsError {
    pub value: HlsErrorValue,
}

#[derive(Debug, Fail)]
pub enum HlsErrorValue {
    #[fail(display = "server error")]
    Error,
    #[fail(display = "session error")]
    SessionError(SessionError),

    #[fail(display = "amf write error")]
    Amf0WriteError(Amf0WriteError),
    #[fail(display = "metadata error")]
    MetadataError(MetadataError),
    #[fail(display = "flv demuxer error")]
    FlvDemuxerError(FlvDemuxerError),
    #[fail(display = "media error")]
    MediaError(MediaError),
}

impl From<MediaError> for HlsError {
    fn from(error: MediaError) -> Self {
        HlsError {
            value: HlsErrorValue::MediaError(error),
        }
    }
}

impl From<SessionError> for HlsError {
    fn from(error: SessionError) -> Self {
        HlsError {
            value: HlsErrorValue::SessionError(error),
        }
    }
}

impl From<FlvDemuxerError> for HlsError {
    fn from(error: FlvDemuxerError) -> Self {
        HlsError {
            value: HlsErrorValue::FlvDemuxerError(error),
        }
    }
}

impl From<Amf0WriteError> for HlsError {
    fn from(error: Amf0WriteError) -> Self {
        HlsError {
            value: HlsErrorValue::Amf0WriteError(error),
        }
    }
}

impl From<MetadataError> for HlsError {
    fn from(error: MetadataError) -> Self {
        HlsError {
            value: HlsErrorValue::MetadataError(error),
        }
    }
}

impl fmt::Display for HlsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}