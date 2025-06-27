
# Backend - PuntoIA Challenge

Backend , construido en **Rust** utilizando el framework **Axum**. Expone dos endpoints REST simples con temática financiera.

## 📡 Endpoints disponibles

| Método | Ruta               | Descripción                            |
|--------|--------------------|----------------------------------------|
| GET    | `/exchange-rates` | Retorna tasas de cambio simuladas USD/EUR → COP |
| GET    | `/summary`        | Retorna un resumen de ingresos, gastos y saldo  |


## Comandos
- cargo run
- El servidor correrá en:
http://127.0.0.1:3000
✅ Recomendación: Al conectar con React Native en Expo, usá tu IP local (por ejemplo, http://192.168.xx.xx:3000) para que el dispositivo móvil pueda acceder al backend.
