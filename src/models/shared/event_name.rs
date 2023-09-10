use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventName {
    pub event_name: EventNameType,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Hash, Clone, Copy, EnumString, Display)]
pub enum EventNameType {
    #[strum(serialize = "Legacy.AudioPlayerGui.LyricsViewedEvent")]
    LegacyAudioPlayerGuiLyricsViewedEvent,
    #[strum(serialize = "Legacy.ListModel.DeleteItemRequest")]
    LegacyListModelDeleteItemRequest,
    #[strum(serialize = "Legacy.MediaPlayer.SequenceModified")]
    LegacyMediaPlayerSequenceModified,
    #[strum(serialize = "Legacy.PlaybackController.ButtonCommand")]
    LegacyPlaybackControllerButtonCommand,
    #[strum(serialize = "EffectsController.RequestEffectChangeRequest")]
    EffectsControllerRequestEffectChangeRequest,
    #[strum(serialize = "Legacy.ExternalMediaPlayer.RequestToken")]
    LegacyExternalMediaPlayerRequestToken,
    #[strum(serialize = "ITEMS_UPDATED")]
    ItemsUpdated,
    #[strum(serialize = "Alexa.Video.Xray.ShowDetailsSuccessful")]
    AlexaVideoXrayShowDetailsSuccessful,
    #[strum(serialize = "PlaybackController.NextCommandIssued")]
    PlaybackControllerNextCommandIssued,
    #[strum(serialize = "Legacy.MediaPlayer.PlaybackFinished")]
    LegacyMediaPlayerPlaybackFinished,
    #[strum(serialize = "Alexa.Camera.VideoCaptureController.CaptureFailed")]
    AlexaCameraVideoCaptureControllerCaptureFailed,
    #[strum(serialize = "SKILL_DISABLED")]
    SkillDisabled,
    #[strum(serialize = "Alexa.Camera.VideoCaptureController.CancelCaptureFailed")]
    AlexaCameraVideoCaptureControllerCancelCaptureFailed,
    #[strum(serialize = "CustomInterfaceController.EventsReceived")]
    CustomInterfaceControllerEventsReceived,
    #[strum(serialize = "Legacy.DeviceNotification.NotificationStarted")]
    LegacyDeviceNotificationNotificationStarted,
    #[strum(serialize = "REMINDER_UPDATED")]
    ReminderUpdated,
    #[strum(serialize = "AUDIO_ITEM_PLAYBACK_STOPPED")]
    AudioItemPlaybackStopped,
    #[strum(serialize = "Legacy.AuxController.InputActivityStateChanged")]
    LegacyAuxControllerInputActivityStateChanged,
    #[strum(serialize = "LocalApplication.MShopPurchasing.Event")]
    LocalApplicationMShopPurchasingEvent,
    #[strum(serialize = "Legacy.ExternalMediaPlayer.AuthorizationComplete")]
    LegacyExternalMediaPlayerAuthorizationComplete,
    #[strum(serialize = "LocalApplication.HHOPhotos.Event")]
    LocalApplicationHHOPhotosEvent,
    #[strum(serialize = "Alexa.Presentation.APL.UserEvent")]
    AlexaPresentationAPLUserEvent,
    #[strum(serialize = "Legacy.AudioPlayer.PlaybackInterrupted")]
    LegacyAudioPlayerPlaybackInterrupted,
    #[strum(serialize = "Legacy.BluetoothNetwork.DeviceUnpairFailure")]
    LegacyBluetoothNetworkDeviceUnpairFailure,
    #[strum(serialize = "IN_SKILL_PRODUCT_SUBSCRIPTION_ENDED")]
    InSkillProductSubscriptionEnded,
    #[strum(serialize = "Alexa.FileManager.UploadController.UploadFailed")]
    AlexaFileManagerUploadControllerUploadFailed,
    #[strum(serialize = "Legacy.BluetoothNetwork.DeviceConnectedFailure")]
    LegacyBluetoothNetworkDeviceConnectedFailure,
    #[strum(serialize = "Legacy.AudioPlayer.AudioStutter")]
    LegacyAudioPlayerAudioStutter,
    #[strum(serialize = "Alexa.Camera.VideoCaptureController.CaptureStarted")]
    AlexaCameraVideoCaptureControllerCaptureStarted,
    #[strum(serialize = "Legacy.Speaker.MuteChanged")]
    LegacySpeakerMuteChanged,
    #[strum(serialize = "CardRenderer.DisplayContentFinished")]
    CardRendererDisplayContentFinished,
    #[strum(serialize = "Legacy.SpeechSynthesizer.SpeechStarted")]
    LegacySpeechSynthesizerSpeechStarted,
    #[strum(serialize = "AudioPlayer.PlaybackStopped")]
    AudioPlayerPlaybackStopped,
    #[strum(serialize = "Legacy.SoftwareUpdate.CheckSoftwareUpdateReport")]
    LegacySoftwareUpdateCheckSoftwareUpdateReport,
    #[strum(serialize = "CardRenderer.DisplayContentStarted")]
    CardRendererDisplayContentStarted,
    #[strum(serialize = "LocalApplication.NotificationsApp.Event")]
    LocalApplicationNotificationsAppEvent,
    #[strum(serialize = "AudioPlayer.PlaybackStarted")]
    AudioPlayerPlaybackStarted,
    #[strum(serialize = "Legacy.DeviceNotification.NotificationEnteredForground")]
    LegacyDeviceNotificationNotificationEnteredForground,
    #[strum(serialize = "Legacy.DeviceNotification.SetNotificationFailed")]
    LegacyDeviceNotificationSetNotificationFailed,
    #[strum(serialize = "Legacy.AudioPlayer.PeriodicPlaybackProgressReport")]
    LegacyAudioPlayerPeriodicPlaybackProgressReport,
    #[strum(serialize = "Legacy.HomeAutoWifiController.HttpNotified")]
    LegacyHomeAutoWifiControllerHttpNotified,
    #[strum(serialize = "Alexa.Camera.PhotoCaptureController.CancelCaptureFailed")]
    AlexaCameraPhotoCaptureControllerCancelCaptureFailed,
    #[strum(serialize = "SKILL_ACCOUNT_LINKED")]
    SkillAccountLinked,
    #[strum(serialize = "LocalApplication.AlexaVision.Event")]
    LocalApplicationAlexaVisionEvent,
    #[strum(serialize = "LocalApplication.Closet.Event")]
    LocalApplicationClosetEvent,
    #[strum(serialize = "Alexa.FileManager.UploadController.CancelUploadFailed")]
    AlexaFileManagerUploadControllerCancelUploadFailed,
    #[strum(serialize = "Legacy.MediaPlayer.PlaybackResumed")]
    LegacyMediaPlayerPlaybackResumed,
    #[strum(serialize = "SKILL_PERMISSION_ACCEPTED")]
    SkillPermissionAccepted,
    #[strum(serialize = "FitnessSessionController.FitnessSessionPaused")]
    FitnessSessionControllerFitnessSessionPaused,
    #[strum(serialize = "Legacy.AudioPlayer.PlaybackPaused")]
    LegacyAudioPlayerPlaybackPaused,
    #[strum(serialize = "Alexa.Presentation.HTML.LifecycleStateChanged")]
    AlexaPresentationHTMLLifecycleStateChanged,
    #[strum(serialize = "LocalApplication.SipUserAgent.Event")]
    LocalApplicationSipUserAgentEvent,
    #[strum(serialize = "Legacy.MediaPlayer.PlaybackStarted")]
    LegacyMediaPlayerPlaybackStarted,
    #[strum(serialize = "REMINDER_STATUS_CHANGED")]
    ReminderStatusChanged,
    #[strum(serialize = "MessagingController.UploadConversations")]
    MessagingControllerUploadConversations,
    #[strum(serialize = "ITEMS_DELETED")]
    ItemsDeleted,
    #[strum(serialize = "Legacy.AuxController.PluggedStateChanged")]
    LegacyAuxControllerPluggedStateChanged,
    #[strum(serialize = "Legacy.AudioPlayer.PlaybackStarted")]
    LegacyAudioPlayerPlaybackStarted,
    #[strum(serialize = "Alexa.FileManager.UploadController.UploadStarted")]
    AlexaFileManagerUploadControllerUploadStarted,
    #[strum(serialize = "ITEMS_CREATED")]
    ItemsCreated,
    #[strum(serialize = "Legacy.ExternalMediaPlayer.Event")]
    LegacyExternalMediaPlayerEvent,
    #[strum(serialize = "LocalApplication.LocalMediaPlayer.Event")]
    LocalApplicationLocalMediaPlayerEvent,
    #[strum(serialize = "LocalApplication.KnightContacts.Event")]
    LocalApplicationKnightContactsEvent,
    #[strum(serialize = "LocalApplication.Calendar.Event")]
    LocalApplicationCalendarEvent,
    #[strum(serialize = "Legacy.AlertsController.DismissCommand")]
    LegacyAlertsControllerDismissCommand,
    #[strum(serialize = "Legacy.AudioPlayer.PlaybackStutterFinished")]
    LegacyAudioPlayerPlaybackStutterFinished,
    #[strum(serialize = "Legacy.SpeechSynthesizer.SpeechFinished")]
    LegacySpeechSynthesizerSpeechFinished,
    #[strum(serialize = "Legacy.ExternalMediaPlayer.ReportDiscoveredPlayers")]
    LegacyExternalMediaPlayerReportDiscoveredPlayers,
    #[strum(serialize = "LocalApplication.SipClient.Event")]
    LocalApplicationSipClientEvent,
    #[strum(serialize = "Legacy.BluetoothNetwork.DeviceUnpairSuccess")]
    LegacyBluetoothNetworkDeviceUnpairSuccess,
    #[strum(serialize = "Legacy.Speaker.VolumeChanged")]
    LegacySpeakerVolumeChanged,
    #[strum(serialize = "CardRenderer.ReadContentFinished")]
    CardRendererReadContentFinished,
    #[strum(serialize = "LocalApplication.HomeAutomationMedia.Event")]
    LocalApplicationHomeAutomationMediaEvent,
    #[strum(serialize = "Legacy.BluetoothNetwork.CancelPairingMode")]
    LegacyBluetoothNetworkCancelPairingMode,
    #[strum(serialize = "LocalApplication.DigitalDash.Event")]
    LocalApplicationDigitalDashEvent,
    #[strum(serialize = "CardRenderer.ReadContentStarted")]
    CardRendererReadContentStarted,
    #[strum(serialize = "Legacy.GameEngine.GameInputEvent")]
    LegacyGameEngineGameInputEvent,
    #[strum(serialize = "LocalApplication.LocalVoiceUI.Event")]
    LocalApplicationLocalVoiceUIEvent,
    #[strum(serialize = "Legacy.Microphone.AudioRecording")]
    LegacyMicrophoneAudioRecording,
    #[strum(serialize = "LocalApplication.AlexaPlatformTestSpeechlet.Event")]
    LocalApplicationAlexaPlatformTestSpeechletEvent,
    #[strum(serialize = "Legacy.HomeAutoWifiController.SsdpServiceDiscovered")]
    LegacyHomeAutoWifiControllerSsdpServiceDiscovered,
    #[strum(serialize = "Alexa.Camera.PhotoCaptureController.CancelCaptureFinished")]
    AlexaCameraPhotoCaptureControllerCancelCaptureFinished,
    #[strum(serialize = "Legacy.HomeAutoWifiController.DeviceReconnected")]
    LegacyHomeAutoWifiControllerDeviceReconnected,
    #[strum(serialize = "SKILL_ENABLED")]
    SkillEnabled,
    #[strum(serialize = "Alexa.Camera.VideoCaptureController.CancelCaptureFinished")]
    AlexaCameraVideoCaptureControllerCancelCaptureFinished,
    #[strum(serialize = "MessagingController.UpdateMessagesStatusRequest")]
    MessagingControllerUpdateMessagesStatusRequest,
    #[strum(serialize = "REMINDER_STARTED")]
    ReminderStarted,
    #[strum(serialize = "CustomInterfaceController.Expired")]
    CustomInterfaceControllerExpired,
    #[strum(serialize = "LocalApplication.AvaPhysicalShoppingEvent")]
    LocalApplicationAvaPhysicalShoppingEvent,
    #[strum(serialize = "LocalApplication.WebVideoPlayer.Event")]
    LocalApplicationWebVideoPlayerEvent,
    #[strum(serialize = "Legacy.HomeAutoWifiController.SsdpServiceTerminated")]
    LegacyHomeAutoWifiControllerSsdpServiceTerminated,
    #[strum(serialize = "LocalApplication.FireflyShoppingEvent")]
    LocalApplicationFireflyShoppingEvent,
    #[strum(serialize = "Legacy.PlaybackController.NextCommand")]
    LegacyPlaybackControllerNextCommand,
    #[strum(serialize = "LocalApplication.Gallery.Event")]
    LocalApplicationGalleryEvent,
    #[strum(serialize = "Alexa.Presentation.PresentationDismissed")]
    AlexaPresentationPresentationDismissed,
    #[strum(serialize = "EffectsController.StateReceiptChangeRequest")]
    EffectsControllerStateReceiptChangeRequest,
    #[strum(serialize = "LocalApplication.AlexaTranslationLiveTranslationEvent")]
    LocalApplicationAlexaTranslationLiveTranslationEvent,
    #[strum(serialize = "LocalApplication.AlexaNotificationsEvent")]
    LocalApplicationAlexaNotificationsEvent,
    #[strum(serialize = "REMINDER_DELETED")]
    ReminderDeleted,
    #[strum(serialize = "GameEngine.InputHandlerEvent")]
    GameEngineInputHandlerEvent,
    #[strum(serialize = "Legacy.PlaylistController.Response")]
    LegacyPlaylistControllerResponse,
    #[strum(serialize = "LocalApplication.KnightHome.Event")]
    LocalApplicationKnightHomeEvent,
    #[strum(serialize = "Legacy.ListRenderer.ListItemEvent")]
    LegacyListRendererListItemEvent,
    #[strum(serialize = "AudioPlayer.PlaybackFailed")]
    AudioPlayerPlaybackFailed,
    #[strum(serialize = "LocalApplication.KnightHomeThingsToTryEvent")]
    LocalApplicationKnightHomeThingsToTryEvent,
    #[strum(serialize = "Legacy.BluetoothNetwork.SetDeviceCategoriesFailed")]
    LegacyBluetoothNetworkSetDeviceCategoriesFailed,
    #[strum(serialize = "Legacy.ExternalMediaPlayer.Logout")]
    LegacyExternalMediaPlayerLogout,
    #[strum(serialize = "Alexa.FileManager.UploadController.UploadFinished")]
    AlexaFileManagerUploadControllerUploadFinished,
    #[strum(serialize = "Legacy.ActivityManager.FocusChanged")]
    LegacyActivityManagerFocusChanged,
    #[strum(serialize = "Legacy.AlertsController.SnoozeCommand")]
    LegacyAlertsControllerSnoozeCommand,
    #[strum(serialize = "Legacy.SpeechRecognizer.WakeWordChanged")]
    LegacySpeechRecognizerWakeWordChanged,
    #[strum(serialize = "Legacy.ListRenderer.GetListPageByTokenRequest")]
    LegacyListRendererGetListPageByTokenRequest,
    #[strum(serialize = "MessagingController.UpdateSendMessageStatusRequest")]
    MessagingControllerUpdateSendMessageStatusRequest,
    #[strum(serialize = "FitnessSessionController.FitnessSessionEnded")]
    FitnessSessionControllerFitnessSessionEnded,
    #[strum(serialize = "Alexa.Presentation.APL.RuntimeError")]
    AlexaPresentationAPLRuntimeError,
    #[strum(serialize = "Legacy.ListRenderer.GetListPageByOrdinal")]
    LegacyListRendererGetListPageByOrdinal,
    #[strum(serialize = "FitnessSessionController.FitnessSessionResumed")]
    FitnessSessionControllerFitnessSessionResumed,
    #[strum(serialize = "IN_SKILL_PRODUCT_SUBSCRIPTION_RENEWED")]
    InSkillProductSubscriptionRenewed,
    #[strum(serialize = "Legacy.DeviceNotification.DeleteNotificationSucceeded")]
    LegacyDeviceNotificationDeleteNotificationSucceeded,
    #[strum(serialize = "Legacy.SpeechSynthesizer.SpeechSynthesizerError")]
    LegacySpeechSynthesizerSpeechSynthesizerError,
    #[strum(serialize = "Alexa.VideoXray.ShowDetailsFailed")]
    AlexaVideoXrayShowDetailsFailed,
    #[strum(serialize = "Alexa.FileManager.UploadController.CancelUploadFinished")]
    AlexaFileManagerUploadControllerCancelUploadFinished,
    #[strum(serialize = "Legacy.SconeRemoteControl.PlayPause")]
    LegacySconeRemoteControlPlayPause,
    #[strum(serialize = "Legacy.DeviceNotification.NotificationEnteredBackground")]
    LegacyDeviceNotificationNotificationEnteredBackground,
    #[strum(serialize = "SKILL_PERMISSION_CHANGED")]
    SkillPermissionChanged,
    #[strum(serialize = "Legacy.AudioPlayer.Metadata")]
    LegacyAudioPlayerMetadata,
    #[strum(serialize = "Legacy.AudioPlayer.PlaybackStutterStarted")]
    LegacyAudioPlayerPlaybackStutterStarted,
    #[strum(serialize = "AUDIO_ITEM_PLAYBACK_FINISHED")]
    AudioItemPlaybackFinished,
    #[strum(serialize = "EffectsController.RequestGuiChangeRequest")]
    EffectsControllerRequestGuiChangeRequest,
    #[strum(serialize = "FitnessSessionController.FitnessSessionStarted")]
    FitnessSessionControllerFitnessSessionStarted,
    #[strum(serialize = "Legacy.PlaybackController.LyricsViewedEvent")]
    LegacyPlaybackControllerLyricsViewedEvent,
    #[strum(serialize = "Legacy.ExternalMediaPlayer.Login")]
    LegacyExternalMediaPlayerLogin,
    #[strum(serialize = "PlaybackController.PauseCommandIssued")]
    PlaybackControllerPauseCommandIssued,
    #[strum(serialize = "Legacy.MediaPlayer.PlaybackIdle")]
    LegacyMediaPlayerPlaybackIdle,
    #[strum(serialize = "Legacy.SconeRemoteControl.Previous")]
    LegacySconeRemoteControlPrevious,
    #[strum(serialize = "DeviceSetup.SetupCompleted")]
    DeviceSetupSetupCompleted,
    #[strum(serialize = "Legacy.MediaPlayer.PlaybackNearlyFinished")]
    LegacyMediaPlayerPlaybackNearlyFinished,
    #[strum(serialize = "LocalApplication.TodoRenderer.Event")]
    LocalApplicationTodoRendererEvent,
    #[strum(serialize = "Legacy.BluetoothNetwork.SetDeviceCategoriesSucceeded")]
    LegacyBluetoothNetworkSetDeviceCategoriesSucceeded,
    #[strum(serialize = "Legacy.BluetoothNetwork.MediaControlSuccess")]
    LegacyBluetoothNetworkMediaControlSuccess,
    #[strum(serialize = "Legacy.MediaGrouping.GroupSyncEvent")]
    LegacyMediaGroupingGroupSyncEvent,
    #[strum(serialize = "Legacy.FavoritesController.Error")]
    LegacyFavoritesControllerError,
    #[strum(serialize = "Legacy.ListModel.GetPageByTokenRequest")]
    LegacyListModelGetPageByTokenRequest,
    #[strum(serialize = "Legacy.ActivityManager.ActivityInterrupted")]
    LegacyActivityManagerActivityInterrupted,
    #[strum(serialize = "Legacy.MeetingClientController.Event")]
    LegacyMeetingClientControllerEvent,
    #[strum(serialize = "Legacy.Presentation.PresentationDismissedEvent")]
    LegacyPresentationPresentationDismissedEvent,
    #[strum(serialize = "Legacy.Spotify.Event")]
    LegacySpotifyEvent,
    #[strum(serialize = "Legacy.ExternalMediaPlayer.Error")]
    LegacyExternalMediaPlayerError,
    #[strum(serialize = "Legacy.AuxController.DirectionChanged")]
    LegacyAuxControllerDirectionChanged,
    #[strum(serialize = "AudioPlayer.PlaybackNearlyFinished")]
    AudioPlayerPlaybackNearlyFinished,
    #[strum(serialize = "Alexa.Camera.PhotoCaptureController.CaptureFinished")]
    AlexaCameraPhotoCaptureControllerCaptureFinished,
    #[strum(serialize = "Legacy.UDPController.BroadcastResponse")]
    LegacyUDPControllerBroadcastResponse,
    #[strum(serialize = "Legacy.AudioPlayer.PlaybackResumed")]
    LegacyAudioPlayerPlaybackResumed,
    #[strum(serialize = "Legacy.DeviceNotification.DeleteNotificationFailed")]
    LegacyDeviceNotificationDeleteNotificationFailed,
}
