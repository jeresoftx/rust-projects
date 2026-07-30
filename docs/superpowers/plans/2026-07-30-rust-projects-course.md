# Plan de implementación de Rust Projects

**Estado:** draft  
**Fuente:** RFC-0001 §10, §14, §15, §16 y §20; RFC-0002.

**Representación operativa:** [GitHub Project #21](https://github.com/users/jeresoftx/projects/21).

## Objetivo

Entregar un curso-proyecto de implementaciones educativas en Rust que permita
seguir sistemas reales de principio a fin, con pruebas, capítulos, ejemplos,
diagramas y límites explícitos. Cada sistema es funcional, pero no intenta
copiar la superficie ni las garantías de un producto de producción.

## Arquitectura de entrega

Cada proyecto se divide en tres issues cerrables por PR:

1. especificación: problema, contrato, invariantes, alternativas y límites;
2. modelo: TDD, implementación mínima, diagnósticos y pruebas de borde;
3. capítulo: Mermaid, ejemplos, ejercicios, soluciones y límites honestos.

## Milestones y roadmap estimado

| Milestone | Ventana | Proyectos |
|---|---|---|
| 0. Fundación | 2026-08-03 a 2026-08-04 | Coordinación y trazabilidad. |
| 1. Herramientas CLI | 2026-08-05 a 2026-08-22 | `grep`, `curl`. |
| 2. Servicios de red | 2026-08-25 a 2026-09-12 | HTTP server, load balancer. |
| 3. Sistemas de estado | 2026-09-15 a 2026-10-03 | Redis, SQLite. |
| 4. Herramientas de plataforma | 2026-10-06 a 2026-10-31 | Git, Docker, NGINX. |
| 5. Log distribuido y cierre | 2026-11-03 a 2026-11-21 | Kafka, ruta editorial. |

Las fechas son pronósticos de roadmap, no compromisos de publicación. Los
milestones no tendrán vencimientos artificiales.

## Ruta crítica

`#1 → #2 → #3 → #4 → #5 → #6 → #7 → #8 → #9 → #10 → #11 → #12 → #13 → #14 → #15 → #16 → #17 → #18 → #19 → #20 → #21 → #22 → #23 → #24 → #25 → #26 → #27 → #28 → #29 → #30 → #31 → #32`

El orden refleja la ambición declarada en RFC-0001 §10: primero herramientas
acotadas, después servicios y estado, y finalmente el log distribuido.

## Checklist operativo

### Fundación

- [x] #1 Coordinar plan, Project y trazabilidad de `rust-projects`.

### grep

- [x] #2 Especificar búsqueda, entradas, errores y límites de `grep`.
- [ ] #3 Implementar y probar `grep` educativo.
- [x] #4 Escribir capítulo, diagrama, ejemplos y ejercicios de `grep`.

### curl

- [x] #5 Especificar cliente HTTP, URLs, respuestas y límites de `curl`.
- [ ] #6 Implementar y probar `curl` educativo.
- [x] #7 Escribir capítulo, diagrama, ejemplos y ejercicios de `curl`.

### Servidor HTTP

- [x] #8 Especificar requests, routing, respuestas y límites del servidor HTTP.
- [ ] #9 Implementar y probar servidor HTTP educativo.
- [x] #10 Escribir capítulo, diagrama, ejemplos y ejercicios del servidor HTTP.

### Load balancer

- [x] #11 Especificar backends, health checks, round-robin y límites.
- [ ] #12 Implementar y probar load balancer educativo.
- [x] #13 Escribir capítulo, diagrama, ejemplos y ejercicios del load balancer.

### Redis

- [x] #14 Especificar protocolo, claves, expiración y límites de Redis educativo.
- [ ] #15 Implementar y probar Redis educativo.
- [x] #16 Escribir capítulo, diagrama, ejemplos y ejercicios de Redis.

### SQLite

- [ ] #17 Especificar páginas, tabla mínima, consultas y límites de SQLite educativo.
- [ ] #18 Implementar y probar SQLite educativo.
- [ ] #19 Escribir capítulo, diagrama, ejemplos y ejercicios de SQLite.

### Git

- [ ] #20 Especificar objetos, índice, referencias y límites de Git educativo.
- [ ] #21 Implementar y probar Git educativo.
- [ ] #22 Escribir capítulo, diagrama, ejemplos y ejercicios de Git.

### Docker

- [ ] #23 Especificar imagen, ejecución conceptual, aislamiento y límites de Docker educativo.
- [ ] #24 Implementar y probar Docker educativo.
- [ ] #25 Escribir capítulo, diagrama, ejemplos y ejercicios de Docker.

### NGINX

- [ ] #26 Especificar proxy inverso, routing y límites de NGINX educativo.
- [ ] #27 Implementar y probar NGINX educativo.
- [ ] #28 Escribir capítulo, diagrama, ejemplos y ejercicios de NGINX.

### Kafka y cierre

- [ ] #29 Especificar log, particiones, consumidores y límites de Kafka educativo.
- [ ] #30 Implementar y probar Kafka educativo.
- [ ] #31 Escribir capítulo, diagrama, ejemplos y ejercicios de Kafka.
- [ ] #32 Completar ruta de lectura, glosario y auditoría editorial en `draft`.

## Criterio de cierre

El plan queda completo en `draft` cuando los 32 issues estén cerrados por PRs
trazables, cada sistema tenga especificación, modelo probado y capítulo, y el
Project no tenga ítems pendientes. No se marca contenido como `reviewed` ni
`published` sin revisión humana.

## Siguiente bloque recomendado

`#1 → #2 → #3 → #4`: confirmar coordinación, contrato, modelo y capítulo de
`grep`. No requiere `unsafe`, nightly ni dependencias externas no triviales.

## Límites y dependencias

- Cada modelo depende de su especificación; cada capítulo depende de su modelo.
- El siguiente sistema depende del capítulo anterior para conservar la ruta de
  aprendizaje, aunque el código pueda vivir en módulos independientes.
- Sin `unsafe`, nightly ni dependencias externas no triviales sin autorización.
- La implementación de Docker no intenta crear aislamiento real del sistema
  operativo sin una decisión explícita; se limita a un modelo didáctico.
