pub enum ProtocolContants{
    Null,
    /// 1
    DataPackage,
    /// 2
    ActivityAuthorization,
    /// 3
    TaskSignIn,
    /// 4
    TaskSimpleSignIn,
    /// 5
    ResultSignInOk,
    /// 6
    ResultSignInErr,
    /// 7
    TaskSignUp,
    /// 8
    ResultSignUpOk,
    /// 9
    ResultSignUpErr,
    NotAssigned10,
    NotAssigned11,
    NotAssigned12,
    NotAssigned13,
    NotAssigned14,
    /// 15
    ActivityMessenger,
    /// 16
    TaskSendMessage,
    /// 17
    ResultSendMessageOk,
    /// 18
    ResultSendMessageErr,
    /// 19
    TaskCheckMessages,
    /// 20
    MessageNothing,
    /// 21
    MessageText,
    NotAssigned21,
    NotAssigned22,
    NotAssigned23,
    NotAssigned24,
    /// 25
    ActivityFileDrop,
    /// 26
    TaskFileDrop,
    /// 27
    MessageFileDropOffer,
    /// 28
    MessageFileDropAccepted,
    /// 29
    MessageFileDropDenied,
}

impl ProtocolContants{
    pub fn new(byte:u8)->ProtocolContants{
        unsafe{std::mem::transmute(byte)}
    }
}