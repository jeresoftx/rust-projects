# NGINX educativo: proxy inverso y routing

## Concepto y problema

Un proxy inverso recibe una request pública y decide a qué servicio interno se
envía. El contrato importante es que el prefijo de ruta se resuelva de forma
determinista antes de tocar la red.

## Contrato e invariantes

El modelo ordena rutas por prefijo más largo. Una request selecciona el backend
de la regla más específica; sin regla devuelve un error controlado. Cambiar el
orden de declaración no cambia el resultado cuando los prefijos son distintos.

## Alternativas y decisión

NGINX real incluye configuración declarativa, buffers, TLS, cache y eventos.
Elegimos routing puro en memoria para razonar sobre precedencia sin afirmar que
el modelo actúe como proxy de bytes.

## Límites honestos

No hay sockets, HTTP, TLS, upstreams reales, headers, retries, cache, rate
limiting ni archivos de configuración. No es un servidor NGINX compatible.

## Recorrido

```mermaid
flowchart LR
    A[Request /api/cursos] --> B[Reglas por prefijo]
    B --> C[/]
    B --> D[/api]
    D --> E[Backend API]
```

## Ejemplo

```rust
use rust_projects::nginx::Router;

let router = Router::default().route("/", "web").route("/api", "api");
assert_eq!(router.resolve("/api/cursos"), Ok("api"));
```

## Ejercicios y soluciones orientativas

1. Agrega una ruta exacta. Solución: decide primero si vence a cualquier
   prefijo y pruébalo como regla distinta.
2. Modela headers. Solución: mantén la decisión de backend separada de la
   transformación de request.
3. Diseña reintentos. Solución: requiere un contrato de idempotencia; no todo
   método HTTP debe repetirse.
