use iroh::{Endpoint, NodeId, NodeAddr};
use iroh::endpoint::{BindError, RecvStream, SendStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

type BufReader = tokio::io::BufReader<RecvStream>;
type BufWriter = tokio::io::BufWriter<SendStream>;

static ALPN: &[u8] = "broadside/net/1".as_bytes();

pub struct Node {
    pub reader: BufReader,
    pub writer: BufWriter,
}
impl Node {
    pub fn new(send: SendStream, recv: RecvStream) -> Self {
        Self {
            reader: BufReader::new(recv),
            writer: BufWriter::new(send),
        }
    }

    pub async fn connect(addr: impl Into<NodeAddr>) -> Result<Self, ConnectError> {
        let endpoint = Endpoint::builder()
            .bind().await?;
        let conn = endpoint.connect(addr, ALPN).await?;
        let (send, recv) = conn.open_bi().await?;
        Ok(Self::new(send, recv))
    }

    pub async fn host() -> Result<(Self, NodeId), HostError> {
        let endpoint = Endpoint::builder()
            .alpns(vec![ALPN.to_vec()])
            .bind().await?;
        let id = endpoint.node_id();
        let conn = endpoint.accept().await
            .expect("endpoint closed")
            .await?;
        let (send, recv) = conn.accept_bi().await?;
        Ok((Self::new(send, recv), id))
    }
}
impl Node {
    pub async fn read(&mut self) -> std::io::Result<Vec<u8>> {
        let count = self.reader.read_u32().await? as usize;
        let mut buf = vec![0u8; count];
        self.reader.read_exact(&mut buf).await?;
        Ok(buf)
    }

    pub async fn write(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        self.writer.write_u32(bytes.len() as u32).await?;
        self.writer.write_all(bytes).await?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error, from_variants::FromVariants)]
pub enum ConnectError {
    #[error("BindError: {0:?}")]
    Bind(BindError),
    #[error("ConnectError: {0:?}")]
    Connect(iroh::endpoint::ConnectError),
    #[error("ConnectionError: {0:?}")]
    EndpointConnection(iroh::endpoint::ConnectionError),
}

#[derive(Debug, thiserror::Error, from_variants::FromVariants)]
pub enum HostError {
    #[error("BindError: {0:?}")]
    Bind(BindError),
    #[error("ConnectionError: {0:?}")]
    EndpointConnection(iroh::endpoint::ConnectionError),
}
