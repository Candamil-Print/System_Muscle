use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Turno {
    pub id_turno: i32,
    pub id_usuario: i32,
    pub id_tipo_turno: i32,
    pub fecha_inicio: String,
    pub fecha_fin: Option<String>,
    pub estado: String,
}

#[derive(Debug, Deserialize)]
pub struct NuevoTurno {
    pub id_usuario: i32,
    pub id_tipo_turno: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TurnoDetalle {
    pub id_turno: i32,
    pub usuario: String,
    pub tipo_turno: String,
    pub hora_inicio: String,
    pub hora_fin: String,
    pub fecha_inicio: String,
    pub fecha_fin: Option<String>,
    pub estado: String,
}

#[derive(Debug, Deserialize)]
pub struct FiltroTurno {
    pub id_usuario: Option<i32>,
    pub estado: Option<String>,
    pub fecha_desde: Option<String>,
    pub fecha_hasta: Option<String>,
}