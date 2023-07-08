use diesel::result::Error as DieselError;
use serde_derive::Serialize;
use std::fmt::{self, Display};
use std::{io::Error as IoError, string::ToString};
use thiserror::Error as ThisError;

///hyper库的返回结果，建议使用anyhow::Result
///如果有兼容性的问题，不使用anyhow::Result返回，可以使用define::error::Result
pub type Result<T, E = ExtraDescError> = core::result::Result<T, E>;

/// 标准错误定义，可以直接使用第二个参数当成错误返回
/// (错误码, 变量名称, 英文说明, 中文说明)
///
/// # Example
///
/// ```ignore
/// use define::error::{Result, Error, ReceiveDataFail};
/// async fn read_line(&self, timeout: u64) -> Result<String> {
///
///     let sess = &self.inner.read().unwrap();
///     if let Some(chan) = &sess.chan {
///         task::block_on(async {
///             if let Err(e) = chan
///                 .read_to_string(&mut data, to, None, self.prompt.as_str())
///                 .await
///             {
///                 self.set_state(ConnState::ReadError).unwrap_or(());
///                 data.push_str(format!("read error: {}", e).as_str());
///                 error!("{} 读取数据失败, error: {:?} sbuf: {}", self, e, &sess.sbuf);
///             }
///         });
///         if data.starts_with("read error") {
///             return Err(ReceiveDataFail.from_desc(format!("读取数据失败 {}", &data)).into());
///         }
///         return Ok(data);
///     }
///
///     self.set_state(ConnState::ReadError).unwrap_or(());
///     Err(ReceiveDataFail.from_desc(format!(
///         "{} read_line error, encounter bad channel.",
///         self
///     )).into())
/// }
/// ```
macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr, $phrase_cn:expr);
        )+
    ) => {
        $(
            $(#[$docs])*
            #[allow(non_upper_case_globals)]
            pub const $konst: Error = Error($num);
        )+

        fn canonical_reason_en(num: u16) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }

        fn canonical_reason_cn(num: u16) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase_cn),
                )+
                _ => None
            }
        }
    }
}

status_codes! {
    //I/O Error 1001-2000
    (1001, FileNotFound, "file not found", "文件未发现");
    (1002, PermissionDenied, "permission denied", "操作被拒绝");
    (1003, ConnectionRefused, "connection refused", "远程服务器连接被拒绝");
    (1004, ConnectionReset, "connection reset", "远程服务器连接被重置");
    (1005, ConnectionAborted, "connection aborted", "远程服务器连接被中止");
    (1006, NotConnected, "not connected", "网络操作失败，没有连接");
    (1007, AddrInUse, "address in use", "Socket地址被占用");
    (1008, AddrNotAvailable, "address not available", "请求的地址不存在");
    (1009, BrokenPipe, "broken pipe", "操作失败，因为管道已关闭");
    (1010, AlreadyExists, "entity already exists", "文件已存在");
    (1011, WouldBlock, "operation would block", "操作需要阻塞才能完成");
    (1012, InvalidInput, "invalid input parameter", "参数错误");
    (1013, InvalidData, "invalid data", "数据无效");
    (1014, TimedOut, "timed out", "操作超时");
    (1015, WriteZero, "write zero", "写入时返回空数据");
    (1016, Interrupted, "operation interrupted", "操作中断");
    (1017, Other, "other os error", "其他I/O错误");
    (1018, UnexpectedEof, "unexpected end of file", "操作需要阻塞才能完成");
    //Message Error 2001-3000
    (2001, InvalidMessageQuque, "invalid message quque", "无效的消息队列类型");
    (2002, ConnectionMessageQuqueError, "connection message quque error", "连接消息队列失败");
    (2003, SubscribeMessageQuqueFail, "subscribe message quque fail", "订阅消息队列失败");
    (2004, FetchMessageFail, "fetch message fail", "获取消息失败");
    (2005, FetchMessageTimeout, "fetch message timeout", "获取消息超时");
    (2006, InvalidMessageData, "invalid message data", "无效的消息格式");
    (2007, InvalidCommand, "invalid command", "无效的消息指令");
    (2008, InvalidUseRule, "invalid use rule", "无效的规则");
    //DataBase Error 3001-4000
    (3001, DataBaseInvalidQuery, "dataBase invalid query", "数据库查询参数错误");
    (3002, DataBaseError, "database error", "数据库返回错误");
    (3003, DataBaseNotFound, "result not found", "没有查询到结果");
    (3101, InvalidConnection, "DataBase Invalid Connection", "数据连接无效");
    //Device Error 4001-5000
    (4001, ConnectionDeviceError, "connection device error", "连接设备失败");
    (4002, ConnectionDeviceTimeout, "connection device timeout", "连接设备超时");
    (4003, DeviceAddrInvalid, "device address invalid", "设备地址无效");
    (4004, DeviceNotFound, "device not found", "设备不存在");
    (4005, InvalidDeviceType, "invalid device type", "不支持的设备类型");
    (4006, SendDataTimeout, "send data timeout", "发送数据超时");
    (4007, SendDataFail, "send data fail", "发送数据失败");
    (4008, InvalidSendData, "invalid send data", "发送数据无效");
    (4009, ReceiveDataTimeout, "receive data timeout", "接收数据超时");
    (4010, ReceiveDataFail, "receive data fail", "接收数据失败");
    (4011, ReceiveUnexpectedEof, "receive unexpected eof", "设备连接异常结束");
    (4012, DeviceAlreadyExist, "device already exist", "设备已存在");
    (4013, DeviceNotUsed, "device not used", "设备不可用");
    (4014, DeviceReportError, "device report error", "设备执行指令报错");
    //System Error 5001-6000
    (5001, UnexpectedErrorOccured, "unexpected error occured", "发生意外错误");
    (5002, ServerRegisterFail, "server register fail", "服务注册失败");
    (5003, ConfigurationInvalid, "configuration invalid", "配置无效");
    (5100, UnKnowError, "unknow error", "未定义错误");
    //Token Error 6001-7000
    (6001, RoleTypeError, "role type error", "权限类型不存在");

    //translate Error 7001-7999
    (7001, TransInitError, "translate init error", "翻译器初始化错误");
    (7002, TransRegisterError, "translate register error", "翻译器注册错误");
    (7003, CheckError, "translate check error", "翻译check错误");
    (7004, TransInnerError, "translate inner error", "翻译内部错误");
}

#[derive(ThisError, Debug)]
pub(crate) struct CustomError {
    pub code: u16,
    pub reason: String,
    pub desc: String,
}

impl Display for CustomError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "error code{} reason:{} desc:{}",
            self.code, self.reason, self.desc
        )
    }
}

pub(crate) trait ErrorMeta {
    fn status_code(&self) -> u16;
    fn reason(&self) -> String;
    fn desc(&self) -> String;

    fn to_std_err(&self) -> CustomError {
        CustomError {
            code: self.status_code(),
            reason: self.reason(),
            desc: self.desc(),
        }
    }
}

#[derive(ThisError, Debug, Serialize, Clone)]
pub struct ExtraDescError {
    #[source]
    pub err: Error,
    pub desc: String,
}

impl Display for ExtraDescError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "error {} desc:{}", self.err, self.desc)
    }
}

impl From<Error> for ExtraDescError {
    fn from(source: Error) -> Self {
        ExtraDescError {
            err: source,
            desc: String::new(),
        }
    }
}

#[derive(ThisError, Debug, PartialEq, Eq, Serialize, Clone)]
pub struct Error(pub u16);

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "error code{} reason:{:?} desc:{:?}",
            self.0,
            self.reason_en(),
            self.reason_cn()
        )
    }
}

impl Error {
    pub fn code(&self) -> u16 {
        self.0
    }

    pub fn reason_en(&self) -> Option<&str> {
        canonical_reason_en(self.0)
    }

    pub fn reason_cn(&self) -> Option<&str> {
        canonical_reason_cn(self.0)
    }

    #[allow(dead_code, clippy::wrong_self_convention)]
    pub fn from_error(self, error: Error) -> ExtraDescError {
        ExtraDescError {
            err: self,
            desc: error.to_string(),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_desc<S: Into<String>>(self, desc: S) -> ExtraDescError {
        ExtraDescError {
            err: self,
            desc: desc.into(),
        }
    }
}

impl From<IoError> for ExtraDescError {
    fn from(e: IoError) -> Self {
        let error = match e.kind() {
            std::io::ErrorKind::NotFound => FileNotFound,
            std::io::ErrorKind::PermissionDenied => PermissionDenied,
            std::io::ErrorKind::ConnectionRefused => ConnectionRefused,
            std::io::ErrorKind::ConnectionReset => ConnectionReset,
            std::io::ErrorKind::ConnectionAborted => ConnectionAborted,
            std::io::ErrorKind::NotConnected => NotConnected,
            std::io::ErrorKind::AddrInUse => AddrInUse,
            std::io::ErrorKind::AddrNotAvailable => AddrNotAvailable,
            std::io::ErrorKind::BrokenPipe => BrokenPipe,
            std::io::ErrorKind::AlreadyExists => AlreadyExists,
            std::io::ErrorKind::WouldBlock => WouldBlock,
            std::io::ErrorKind::InvalidInput => InvalidInput,
            std::io::ErrorKind::InvalidData => InvalidData,
            std::io::ErrorKind::TimedOut => TimedOut,
            std::io::ErrorKind::WriteZero => WriteZero,
            std::io::ErrorKind::Interrupted => Interrupted,
            std::io::ErrorKind::Other => Other,
            std::io::ErrorKind::UnexpectedEof => UnexpectedEof,
            _ => UnKnowError,
        };
        error.into()
    }
}

impl From<serde_json::Error> for ExtraDescError {
    fn from(e: serde_json::Error) -> Self {
        InvalidMessageData.from_desc(e.to_string().as_str())
    }
}

impl From<toml::de::Error> for ExtraDescError {
    fn from(e: toml::de::Error) -> Self {
        ConfigurationInvalid.from_desc(e.to_string().as_str())
    }
}

impl From<DieselError> for ExtraDescError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(_, err) => DataBaseError.from_desc(err.message()),
            DieselError::NotFound => DataBaseNotFound.from_desc(error.to_string()),
            DieselError::QueryBuilderError(err) => DataBaseInvalidQuery.from_desc(err.to_string()),
            err => UnKnowError.from_desc(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[allow(dead_code)]
    #[derive(ThisError, Debug)]
    enum TestError {
        #[error("it's error {0}")]
        Example(String),
    }

    fn try_open_file() -> Result<()> {
        let _file = File::open("path")?;
        Ok(())
    }

    #[test]
    fn test_error() {
        let message = "it's error";
        let error = InvalidCommand.from_desc(message);
        assert_eq!(error.desc, message);
    }

    #[test]
    fn test_io_error() {
        if let Err(error) = try_open_file() {
            assert_eq!(error.err.0, 1001);
        }
    }

    #[derive(Debug, ThisError)]
    pub enum NetError {
        #[error("connect protocol error: {0}")]
        ConnProtoError(String),
    }

    pub type NetResult<T> = Result<T, NetError>;

    fn old_read_line() -> NetResult<()> {
        Err(NetError::ConnProtoError(format!(
            "read_line error, encounter bad channel.",
        )))
    }

    fn new_read_line() -> Result<()> {
        old_read_line().map_err(|error| {
            //error!("")
            ReceiveDataFail.from_desc(&error.to_string()).into()
        })
    }

    #[test]
    fn test_map_error() {
        if let Err(error) = new_read_line() {
            assert_eq!(
                &error.desc,
                "connect protocol error: read_line error, encounter bad channel."
            );
        } else {
            panic!();
        }
    }
}
