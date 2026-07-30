# Entrega y trazabilidad

Este repositorio se entrega como un curso-proyecto en estado `draft`. Cada
unidad educativa conserva una secuencia explícita: especificación, modelo
probado y capítulo. Esta separación evita que el código aparezca sin el
contexto que permite juzgarlo.

## Regla de entrega

Cada issue se resuelve mediante una rama aislada, un commit principal y un PR
que contiene `Closes #N`. El issue y el PR comparten milestone, labels,
asignación a `jeresoftx` e ítem en el GitHub Project del curso.

## Verificación

Los cambios de Rust deben pasar `cargo fmt --check`, Clippy con advertencias
como error, pruebas de todos los targets y doctests. Los cambios documentales
deben pasar `git diff --check` y comprobar enlaces, diagramas Mermaid y
ortografía es-MX.

## Límites editoriales

Los modelos son deliberadamente pequeños: muestran contratos e invariantes,
no sustituyen a sus equivalentes de producción. Ningún capítulo se marca como
`reviewed` o `published` sin revisión humana.
