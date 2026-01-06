use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------
// 1. Enums for Flags (The new part)
// ---------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FunctionFlag {
    Final,
    RequiredAPI,
    BlueprintAuthorityOnly,
    BlueprintCosmetic,
    Net,
    NetReliable,
    NetRequest,
    Exec,
    Native,
    Event,
    NetResponse,
    Static,
    NetMulticast,
    UbergraphFunction,
    MulticastDelegate,
    Public,
    Private,
    Protected,
    Delegate,
    NetServer,
    HasOutParms,
    HasDefaults,
    NetClient,
    DLLImport,
    BlueprintCallable,
    BlueprintEvent,
    BlueprintPure,
    EditorOnly,
    Const,
    NetValidate,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClassFlag {
    Abstract,
    DefaultConfig,
    Config,
    Transient,
    Optional,
    MatchedSerializers,
    ProjectUserConfig,
    Native,
    NotPlaceable,
    PerObjectConfig,
    ReplicationDataIsSetUp,
    EditInlineNew,
    CollapseCategories,
    Interface,
    PerPlatformConfig,
    Const,
    NeedsDeferredDependencyLoading,
    CompiledFromBlueprint,
    MinimalAPI,
    RequiredAPI,
    DefaultToInstanced,
    TokenStreamAssembled,
    HasInstancedReference,
    Hidden,
    Deprecated,
    HideDropDown,
    GlobalUserConfig,
    Intrinsic,
    Constructed,
    ConfigDoNotCheckDefaults,
    NewerVersionExists,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyFlag {
    Edit,
    ConstParm,
    BlueprintVisible,
    ExportObject,
    BlueprintReadOnly,
    Net,
    EditFixedSize,
    Parm,
    OutParm,
    ZeroConstructor,
    ReturnParm,
    DisableEditOnTemplate,
    NonNullable,
    Transient,
    Config,
    RequiredParm,
    DisableEditOnInstance,
    EditConst,
    GlobalConfig,
    InstancedReference,
    ExperimentalExternalObjects,
    DuplicateTransient,
    SaveGame,
    NoClear,
    Virtual,
    ReferenceParm,
    BlueprintAssignable,
    Deprecated,
    IsPlainOldData,
    RepSkip,
    RepNotify,
    Interp,
    NonTransactional,
    EditorOnly,
    NoDestructor,
    AutoWeak,
    ContainsInstancedReference,
    AssetRegistrySearchable,
    SimpleDisplay,
    AdvancedDisplay,
    Protected,
    BlueprintCallable,
    BlueprintAuthorityOnly,
    TextExportTransient,
    NonPIEDuplicateTransient,
    ExposeOnSpawn,
    PersistentInstance,
    UObjectWrapper,
    HasGetValueTypeHash,
    NativeAccessSpecifierPublic,
    NativeAccessSpecifierProtected,
    NativeAccessSpecifierPrivate,
    SkipSerialization,
    TObjectPtr,
    ExperimentalOverridableLogic,
    ExperimentalAlwaysOverriden,
    ExperimentalNeverOverriden,
    AllowSelfReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StructFlag {
    Native,
    IdenticalNative,
    HasInstancedReference,
    NoExport,
    Atomic,
    Immutable,
    AddStructReferencedObjects,
    RequiredAPI,
    NetSerializeNative,
    SerializeNative,
    CopyNative,
    IsPlainOldData,
    NoDestructor,
    ZeroConstructor,
    ExportTextItemNative,
    ImportTextItemNative,
    PostSerializeNative,
    SerializeFromMismatchedTag,
    NetDeltaSerializeNative,
    PostScriptConstruct,
    NetSharedSerialization,
    Trashed,
    NewerVersionExists,
    CanEditChange,
    Visitor,
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum TypeUsageHint {
    UObject,
    ScriptInterface,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "Kind")]
pub enum Type {
    Concrete {
        #[serde(rename = "TypeName")]
        type_name: String,
        #[serde(rename = "UsageHint")]
        usage_hint: Option<TypeUsageHint>,
        #[serde(rename = "ArrayDim")]
        array_dim: usize,
    },
    Container {
        #[serde(rename = "ContainerTypeName")]
        container_type_name: String,
        #[serde(rename = "InnerType")]
        inner_type: Box<Type>,
        #[serde(rename = "ArrayDim")]
        array_dim: usize,
    },
    Map {
        #[serde(rename = "KeyType")]
        key_type: Box<Type>,
        #[serde(rename = "ValueType")]
        value_type: Box<Type>,
        #[serde(rename = "ArrayDim")]
        array_dim: usize,
    },
    Bitfield {
        #[serde(rename = "Offset")]
        offset: u8,
        #[serde(rename = "FieldMask")]
        field_maks: u8,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Type")]
    pub data_type: Type,

    #[serde(rename = "Flags", default)]
    pub flags: HashSet<PropertyFlag>,

    #[serde(rename = "Offset")]
    pub offset: u32,

    #[serde(rename = "Alignment")]
    pub alignment: u32,

    #[serde(rename = "Size")]
    pub size: u32,

    #[serde(rename = "Documentation", default)]
    pub documentation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DelegateKind {
    Single,
    Multicast,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DelegateDefinition {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Size")]
    pub size: u32,
    #[serde(rename = "Alignment")]
    pub alignment: u32,
    #[serde(rename = "Package")]
    pub package: String,
    #[serde(rename = "Kind")]
    pub kind: DelegateKind,
    #[serde(rename = "Function")]
    pub function: Function,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpagueDefinition {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Alignment")]
    pub alignment: u32,
    #[serde(rename = "Size")]
    pub size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumDefinitionEntry {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: i64,
    #[serde(rename = "Documentation")]
    pub documentation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EnumKind {
    EnumClass,
    Namespaced,
    Regular,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumDefinition {
    #[serde(rename = "Name")]
    pub enum_name: String,
    #[serde(rename = "Package")]
    pub package: String,
    #[serde(rename = "Type")]
    pub ty: Type,
    #[serde(rename = "Kind")]
    pub kind: EnumKind,

    #[serde(rename = "Entries")]
    pub entries: Vec<EnumDefinitionEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    #[serde(rename = "Property")]
    pub property: Property,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    #[serde(rename = "FunctionName")]
    pub function_name: String,

    #[serde(rename = "Flags", default)]
    pub flags: HashSet<FunctionFlag>,

    #[serde(rename = "ParamSize")]
    pub param_size: u16,

    #[serde(rename = "Metadata", default)]
    pub meta: HashMap<String, String>,

    #[serde(rename = "Parameters")]
    pub parameters: Vec<ParameterDefinition>,

    #[serde(rename = "Documentation", default)]
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDefinition {
    #[serde(rename = "StructName")]
    pub struct_name: String,

    #[serde(rename = "SuperStruct")]
    pub super_struct: Option<String>,

    #[serde(rename = "Package")]
    pub package: String,

    #[serde(rename = "Metadata", default)]
    pub meta: HashMap<String, String>,
    #[serde(rename = "Flags", default)]
    pub flags: HashSet<StructFlag>,

    #[serde(rename = "MinAlignment")]
    pub min_alignment: u32,

    #[serde(rename = "PropertySizes")]
    pub property_sizes: u32,

    #[serde(rename = "Size")]
    pub size: u32,

    #[serde(rename = "IsPlainOldData")]
    pub is_plain_old_data: bool,

    #[serde(rename = "Properties", default)]
    pub properties: Vec<Property>,

    #[serde(rename = "Documentation", default)]
    pub documentation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassDefinition {
    #[serde(rename = "ClassName")]
    pub class_name: String,

    #[serde(rename = "Package")]
    pub package: String,

    #[serde(rename = "SuperClass")]
    pub super_class: Option<String>,

    #[serde(rename = "IsInterface")]
    pub is_interface: bool,

    #[serde(rename = "Metadata", default)]
    pub meta: HashMap<String, String>,
    #[serde(rename = "Flags", default)]
    pub flags: HashSet<ClassFlag>,

    // Classes also store property sizes/alignment for their instances
    #[serde(rename = "MinAlignment")]
    pub min_alignment: u32,

    #[serde(rename = "PropertySizes")]
    pub property_sizes: u32,

    #[serde(rename = "Properties", default)]
    pub properties: Vec<Property>,

    #[serde(rename = "Functions", default)]
    pub functions: Vec<Function>,

    #[serde(rename = "Documentation", default)]
    pub documentation: Option<String>,
}

// ---------------------------------------------------------
// 5. Root & Tags
// ---------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct TagEntry {
    #[serde(rename = "Tag")]
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Api {
    #[serde(rename = "Tags", default)]
    pub tags: Vec<TagEntry>,
    #[serde(rename = "Structs", default)]
    pub structs: Vec<StructDefinition>,
    #[serde(rename = "Classes", default)]
    pub classes: Vec<ClassDefinition>,
    #[serde(rename = "Enums", default)]
    pub enums: Vec<EnumDefinition>,
    #[serde(rename = "OpagueDefinitions", default)]
    pub opague_defs: Vec<OpagueDefinition>,
    #[serde(rename = "DelegateDefinitions", default)]
    pub delegate_defs: Vec<DelegateDefinition>,
}

impl Api {
    pub fn is_struct_blueprint_type(&self, struct_def: &StructDefinition) -> bool {
        if struct_def.meta.contains_key("BlueprintType") {
            return true;
        }

        // TODO: Might be a bit slow, we should switch to a hashmap for lookup
        if let Some(super_struct) = struct_def.super_struct.as_ref()
            && let Some(super_struct_def) = self
                .structs
                .iter()
                .find(|def| &def.struct_name == super_struct)
        {
            return self.is_struct_blueprint_type(super_struct_def);
        }

        false
    }
    pub fn is_class_blueprint_type(&self, class: &ClassDefinition) -> bool {
        if class.meta.contains_key("BlueprintType") {
            return true;
        }

        // TODO: Might be a bit slow, we should switch to a hashmap for lookup
        if let Some(super_class) = class.super_class.as_ref()
            && let Some(super_class_def) = self
                .classes
                .iter()
                .find(|def| &def.class_name == super_class)
        {
            return self.is_class_blueprint_type(super_class_def);
        }

        false
    }
    pub fn iter_structs(&self) -> impl Iterator<Item = &StructDefinition> {
        self.structs
            .iter()
            .filter(|def| self.is_struct_blueprint_type(def))
    }

    pub fn iter_classes(&self) -> impl Iterator<Item = &ClassDefinition> {
        self.classes.iter()
        // self.classes
        //     .iter()
        //     .filter(|def| self.is_class_blueprint_type(def))
    }
}
