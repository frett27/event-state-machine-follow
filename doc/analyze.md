# Analysis: other event query languages

How the condition sentences in [expressions.md](expressions.md) answer queries that other event languages write differently. The sentences are the `facts.condition` and transition `condition` strings of this crate. They are true or false over one device’s event history.

Five languages were read for this note:

| Language | What a query is | Source |
|----------|-----------------|--------|
| [Elastic EQL](https://www.elastic.co/docs/reference/query-languages/eql/eql-syntax) | A category filter, or an ordered `sequence` of events, optionally inside `maxspan` and joined `by` fields | Elastic EQL syntax reference |
| [Esper EPL patterns](http://esper.espertech.com/release-9.0.0/reference-esper/html/event_patterns.html) | A pattern of events with `and`, `or`, `not`, followed-by `->`, and timers such as `timer:within` and `timer:interval` | Esper 9.0 pattern reference |
| [SiddhiQL](https://siddhi.io/en/v5.1/docs/examples/simple-pattern/) | A streaming query: filter a stream, hold a window, aggregate, or match a pattern with `->` and `within` | Siddhi simple-pattern example; [query guide](https://github.com/siddhi-io/www/blob/master/en/docs/docs/query-guide.md) |
| [Apache Flink CEP](https://nightlies.apache.org/flink/flink-docs-stable/docs/libs/cep/) | A pattern sequence (`begin`, `next`, `followedBy`, `notFollowedBy`) with `where` and one `within` bound | Flink CEP documentation |
| [AWS IoT SQL](https://docs.aws.amazon.com/iot/latest/developerguide/iot-sql-reference.html) | `SELECT … FROM 'topic' WHERE …` on the MQTT message that just arrived | AWS IoT SQL reference |

## What a sentence here evaluates

A sentence matches stored events. It is true when the match list is not empty.

- A name is true when any stored event has that name.
- `name#[attr op value]` keeps events of that name whose own attributes pass the comparison.
- `and`, `or`, and `not` combine those tests. `and` is tighter than `or`. `NOT` covers the rest of the sentence unless parentheses close it.
- `name since 2 hours from now` keeps events with `timestamp >= now - 2 hours`. `now` is the timestamp of the event being evaluated. With `StateAgent`, that event is already in the history.
- `name since 10 minutes from last event of [a, b]` uses the newest of `a` and `b` as the anchor, then keeps events at or after `anchor - 10 minutes`. Events after the anchor still match. The clause has no end time.
- One `StateAgent` holds one device. That partition is the usual stand-in for `by device` or `by host`.

Writing guide: [expressions.md](expressions.md).

## Which query shapes the sentence can fulfill

| Query shape | Their writing | Sentence here | What you get |
|-------------|---------------|---------------|--------------|
| Keep one event whose field passes a test | AWS IoT `WHERE temperature > 50`. Siddhi `TempStream[temp > 40]` | `temperature#[value > 30]` | Any stored reading that passes. Add `since … from now` when the reading must be recent |
| Event has occurred | EQL `enrollment where true` | `enrollment_id_created` | At least one such event in this device’s history |
| Event has never occurred | Esper `not Enrollment` | `NOT enrollment_id_created` | The name is absent from the whole history |
| Both have occurred | Esper `Enrollment and Connection` | `enrollment_id_created AND connection_to_cloud` | Both names are present, in either order |
| Both occurred inside a recent span | EQL `sequence with maxspan=2h` of two events | `enrollment since 2 hours from now AND connection_to_cloud since 2 hours from now` | Both fall in the lookback ending at `now`. Order is not tested |
| Nothing of these names in the recent span | EQL `![ connection ]` inside `maxspan`. Esper `timer:interval and not Connection` | `NOT (connection_to_cloud since 24 hours from now)` | That name is absent from the lookback before `now` |
| Enrolled, and quiet since | — | `enrollment_id_created AND NOT (connection_to_cloud since 24 hours from now)` | The enrollment may be old. The connection must be absent from the last day |
| A later trace relative to an anchor | Esper `A -> B`, Flink `next` / `followedBy`, Siddhi `->` | `heartbeat since 10 minutes from last event of [enrollment]` | Heartbeats older than 10 minutes before the newest enrollment drop out. Heartbeats after that enrollment still match, with no upper bound |
| Any activity recently | EQL `any where true` with a span | `* since 1 hour from now` | Some stored event lies in the lookback |
| This device, not a join key | EQL `sequence by host.name` | One `StateAgent` per subject. In the fleet the id is the device id | The history is already a single object. The sentence does not compare a field of one event to a field of another |

## Rewritten queries

### A hot reading

AWS IoT, on the message that just arrived ([SQL reference](https://docs.aws.amazon.com/iot/latest/developerguide/iot-sql-reference.html)):

```sql
SELECT color AS rgb FROM 'topic/subtopic' WHERE temperature > 50
```

Siddhi, as a stream filter ([query guide](https://github.com/siddhi-io/www/blob/master/en/docs/docs/query-guide.md)):

```sql
from TempStream[(roomNo >= 100 and roomNo < 210) and temp > 40]
select roomNo, temp
insert into HighTempStream;
```

The sentence that keeps the same test on a stored reading:

```text
temperature#[value > 40]
```

That stays true after a later colder reading, because the hot event remains stored. The recent form is:

```text
temperature#[value > 40] since 1 hour from now
```

The sentence does not rename fields and does not insert into another stream. A state that uses the sentence becomes active, and the agent emits `state_activated`. The host routes that.

A range is the same shape Siddhi writes with `and` inside the filter:

```text
temperature#[value > 30 OR value < 5]
```

### Two facts in one span

EQL writes an ordered pair inside a span ([syntax reference](https://www.elastic.co/docs/reference/query-languages/eql/eql-syntax)):

```eql
sequence with maxspan=15m
  [ file where file.extension == "exe" ]
  [ process where true ]
```

Flink writes the same idea as two steps and one bound ([CEP](https://nightlies.apache.org/flink/flink-docs-stable/docs/libs/cep/)):

```text
begin("start").next("end").within(10 seconds)
```

The sentence that requires both names in the lookback, without an order, is:

```text
ConnectionTraceFromDevice#[status == true] since 2 hours from now
AND DataCollectionSuccess#[status == true] since 2 hours from now
```

Use this when either order is enough. An EQL or Flink sequence that is false when the second event comes first has no sentence here.

### Silence after a login

Esper ([pattern reference](http://esper.espertech.com/release-9.0.0/reference-esper/html/event_patterns.html)):

```text
every a=LoginEvent -> (timer:interval(1 min) and not LogoutEvent(userId=a.userId))
```

EQL writes the missing step with `!` and a mandatory `maxspan`. Siddhi writes `not` on a pattern step inside `within`.

For one device, the sentence “no cloud connection in the last day” is:

```text
NOT (connection_to_cloud since 24 hours from now)
```

“Enrolled at some point, and no cloud connection in the last day”:

```text
enrollment_id_created AND NOT (connection_to_cloud since 24 hours from now)
```

This follows the device, not one login id. A logout for a different user on a shared history is not a separate key. Give that user its own agent if the history must be separate. The sentence also does not start a one-minute timer at a particular login. It is re-checked at each arriving event, and `now` is that event’s timestamp.

### A trace measured from another trace

Siddhi ([simple pattern](https://siddhi.io/en/v5.1/docs/examples/simple-pattern/)):

```text
from every (e1 = TemperatureStream) ->
  e2 = TemperatureStream[e1.roomNo == roomNo and (e1.temp + 5) <= temp]
  within 10 min
```

Esper `A -> B` and Flink `followedBy` ask for B after A, and `within` closes the pattern if B is late.

The closest sentence uses the newest anchor:

```text
heartbeat since 10 minutes from last event of [enrollment, connection_to_cloud]
```

A heartbeat 3 hours after that anchor still matches. A heartbeat 11 minutes before it does not. The sentence does not require the heartbeat to be the next event, and it does not compare `e1.temp + 5` with `e2.temp`.

## Queries the sentence does not fulfill

These stay in the language that has them.

| Query | Where it is written | Why there is no sentence |
|-------|---------------------|--------------------------|
| B follows A, and only in that order | EQL `sequence`, Esper `->`, Siddhi `->`, Flink `next` / `followedBy` | `and` is true in either order. `since … from last event of` drops events that are too old before the anchor and keeps every later event |
| B did not follow this A within N | Esper `-> (timer:interval and not B)`, EQL `![…]`, Flink `notFollowedBy` plus `within` | `NOT (B since N from now)` is absence before the current event, not absence after a chosen A |
| The same field value joins two events | EQL `sequence by user.name` and `by file.path` / `by process.executable` | A sentence cannot set one event’s attribute equal to another event’s attribute. Partition by device with one agent |
| Count, max, or a rising value | Siddhi `max(temp)` over `#window.time(10 min)`; a pattern `e1.temp + 5 <= e2.temp` | A sentence tests each event on its own. It has no aggregate and no `e1` / `e2` pair |
| N repeats | Esper `[5] MyEvent`, Flink `times` | The only count is “at least one” |
| Reshape the payload and emit a new stream | AWS IoT `SELECT color AS rgb`, Siddhi `select … insert into` | A sentence is a boolean. The agent emits `state_activated` / `state_deactivated` with the state name. Projection stays in the host |
| Filter only the message in hand, ignoring history | AWS IoT `WHERE temperature > 50` | A passing event stays in the history. A `since` window is how a sentence stops counting it |
| A span that runs forward from an event and then expires | Esper `timer:within`, Flink `within`, Siddhi `within 10 min` | `since` subtracts the duration from the anchor. The word `within` is reserved here and has no sentence form |

## Where to put a fulfilled query

A fulfilled query is one activation sentence, or one transition sentence. Several sentences stay active together, which is how one object holds “enrolled” and “hot” at once. The runnable form of that picture is `cargo run -p event_statemachine_follow --example factory_display`.
