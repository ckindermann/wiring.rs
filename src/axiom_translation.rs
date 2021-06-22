use crate::class_translation as class_translation; 

pub fn translate_subclass_of_axiom(sub: &str, sup: &str) -> String {

    let subclass: class_translation::OWL = serde_json::from_str(sub).unwrap(); 
    let superclass: class_translation::OWL = serde_json::from_str(sup).unwrap(); 

    let lhs : String = class_translation::translate(&subclass);
    let rhs: String = class_translation::translate(&superclass); 
    let expression = format!("[\"SubClassOf\",{},{}]", lhs, rhs);
    expression 
}

pub fn translate_equivalent_class(sub: &str, sup: &str) -> String {

    let subclass: class_translation::OWL = serde_json::from_str(sub).unwrap(); 
    let superclass: class_translation::OWL = serde_json::from_str(sup).unwrap(); 

    let lhs : String = class_translation::translate(&subclass);
    let rhs: String = class_translation::translate(&superclass); 
    let expression = format!("[\"EquivalentClasses\",{},{}]", lhs, rhs);
    expression 
}

pub fn translate_disjoint_classes(ops: &str) -> String {

    let operands : class_translation::OWL = serde_json::from_str(ops).unwrap(); 
    let arguments: String = class_translation::translate(&operands); 
    let expression = format!("[\"DisjointClassses\",{}]", arguments);
    expression 
}

pub fn translate_disjoint_union(u: &str, ops: &str) -> String {

    let union: class_translation::OWL = serde_json::from_str(u).unwrap(); 
    let operands: class_translation::OWL = serde_json::from_str(ops).unwrap(); 

    let lhs : String = class_translation::translate(&union);
    let rhs: String = class_translation::translate(&operands); 
    let expression = format!("[\"DisjointUnionOf\",{},{}]", lhs, rhs);
    expression 
}


