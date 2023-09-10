use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Interface {
    AudioPlayer,
    VideoApp,
    RenderTemplate,
    GameEngine,
    GadgetController,
    CanFulfillIntentRequest,
    AlexaPresentationApl,
    AlexaCameraPhotoCaptureController,
    AlexaCameraVideoCaptureController,
    AlexaFileManagerUploadController,
    CustomInterface,
    AlexaAugmentationEffectsController,
    AppLinks,
    AlexaExtension,
    AppLinksV2,
    AlexaSearch,
    AlexaDatastorePackagemanager,
    AlexaDataStore,
}
