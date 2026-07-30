# Balanceador educativo: backends y round-robin

## Concepto y problema

Un balanceador separa una entrada estable de varios backends. Su decisión debe
ser explicable: una petición no debe elegir un destino muerto ni depender de un
orden accidental de memoria.

## Contrato e invariantes

El modelo conserva una lista ordenada de backends con estado de salud. Cada
selección round-robin escoge el siguiente backend saludable y avanza el cursor
una vez. Si no existe ninguno saludable, devuelve un error controlado. La
selección no ejecuta health checks: recibe su resultado como estado explícito.

## Alternativas y decisión

Least-connections, peso y hashing consistente necesitan métricas o identidad
de cliente. Elegimos round-robin para aislar la invariante de justicia básica
antes de mezclar observabilidad y políticas de afinidad.

## Límites honestos

No hay proxy de bytes, reintentos, timeouts, health checks activos, pesos ni
concurrencia. Es un scheduler determinista, no un balanceador de producción.
