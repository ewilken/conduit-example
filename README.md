carrier conduit example
------------------------


this is example code for customer conduits.

A conduit is a carrier client that subscribes to all devices on a shadow and streams data from them.
You can also poll device endpoints with an interval.
note that polling is not like http polling here, as its still done inside the same carrier stream.

For convenience, most common functionality is implemented into the carrier library,
and customers only need to implement handling the data.



scaling
--------
you cannot run multiple conduits using the same identity. Every conduit instance needs its own identity.
Currently the conduit library also lacks pull coordination for multiple subscribers,
so you will receive all messages at all conduits. But this is of course possible using a different carrier client implementation.
