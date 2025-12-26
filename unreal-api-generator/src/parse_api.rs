use serde::{Deserialize, Serialize};

// ---------------------------------------------------------
// 1. Enums for Flags (The new part)
// ---------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StructFlag {
    Native,
    Atomic,
    Immutable,
    NoExport,
    BlueprintType,

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum PropertyFlag {
    EditAnywhere,
    EditDefaultsOnly,
    EditInstanceOnly,
    BlueprintReadOnly,
    BlueprintReadWrite,
    Transient,
    ConstParm,
    ReturnParm,
    OutParm,

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ClassFlag {
    Abstract,
    DefaultConfig,
    Config,
    Transient,
    MatchedSerializers,
    ProjectUserConfig,
    Native,
    BlueprintType,
    // Safety net
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "Kind")]
pub enum Type {
    Concrete {
        #[serde(rename = "TypeName")]
        type_name: String,
    },
    Container {
        #[serde(rename = "ContainerTypeName")]
        container_type_name: String,
        #[serde(rename = "InnerType")]
        inner_type: Box<Type>,
    },
    Map {
        #[serde(rename = "KeyType")]
        key_type: Box<Type>,
        #[serde(rename = "ValueType")]
        value_type: Box<Type>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Type")]
    pub data_type: Type,

    // CHANGED: Now a Vec of Enums instead of Strings
    #[serde(rename = "Flags", default)]
    pub flags: Vec<PropertyFlag>,

    #[serde(rename = "Offset")]
    pub offset: u32,

    #[serde(rename = "Documentation", default)]
    pub documentation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumDefinitionEntry {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: i64,
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
    #[serde(rename = "Type")]
    pub ty: Type,
    #[serde(rename = "Kind")]
    pub kind: EnumKind,

    #[serde(rename = "Entries")]
    pub entries: Vec<EnumDefinitionEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructDefinition {
    #[serde(rename = "StructName")]
    pub struct_name: String,

    #[serde(rename = "Package")]
    pub package: String,

    #[serde(rename = "Flags", default)]
    pub flags: Vec<StructFlag>,

    #[serde(rename = "MinAlignment")]
    pub min_alignment: u32,

    #[serde(rename = "PropertySizes")]
    pub property_sizes: u32,

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

    // NEW: Classes often have a Super Class
    #[serde(rename = "SuperClass", default)]
    pub super_class: Option<String>,

    #[serde(rename = "Flags", default)]
    pub flags: Vec<ClassFlag>,

    // Classes also store property sizes/alignment for their instances
    #[serde(rename = "MinAlignment")]
    pub min_alignment: u32,

    #[serde(rename = "PropertySizes")]
    pub property_sizes: u32,

    #[serde(rename = "Properties", default)]
    pub properties: Vec<Property>,

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
}
