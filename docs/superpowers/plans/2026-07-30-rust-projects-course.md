# Plan de implementación de Rust Projects

**Estado:** draft  
**Fuente:** RFC-0001 §10, §14, §15, §16 y §20; RFC-0002.

## Objetivo

Entregar un curso-proyecto de implementaciones educativas en Rust que permita
seguir sistemas reales de principio a fin, con pruebas, capítulos, ejemplos,
diagramas y límites explícitos.

## Checklist de fases

- [ ] Fundación: Project, milestones, labels, ruta crítica e issues.
- [ ] `grep`: búsqueda, archivos, errores y capítulo.
- [ ] `curl`: HTTP cliente, protocolos mínimos y capítulo.
- [ ] Servidor HTTP y load balancer: routing, concurrencia y capítulo.
- [ ] Redis educativo: protocolo, almacenamiento y capítulo.
- [ ] SQLite educativo: páginas, consultas mínimas y capítulo.
- [ ] Git educativo: objetos, índices y capítulo.
- [ ] Docker y NGINX educativos: aislamiento conceptual, proxy y capítulo.
- [ ] Kafka educativo: log, particiones, consumidores y capítulo.
- [ ] Cierre editorial: ruta de lectura, glosario y auditoría en `draft`.

## Reglas de ejecución

Cada fase se separa en especificación, modelo probado y capítulo. Antes de
código habrá issues asignados a `jeresoftx`, milestones, labels y Project.
No se agrega `unsafe`, nightly ni una dependencia no trivial sin autorización.
