# How does it work?

- The `refraction-rdp-priv` (henceforth `priv`) process must be started first as `root` (or with appropriate capabilities). `priv` creates a socket depending on the configuration file: `/etc/refraction-rdp/refraction.conf`, issues a readiness notification of FD 1, and then listens on the socket.

- Then permissions of the socket must be changed to allow permitted users to read and write to it e.g. using `ExecStartPost` and `setfacl`.

- Then either `refraction-rdp-client` (henceforth `client`) or `refraction-rdp-server` (henceforth `server`). Whichever of these is started, it then creates a user namespace and gives itself priviledges within the namespace. It then makes a request on the socket created by `priv` (consisting of pid + 'c' or 's' depending on whether it is a client or server e.g. '3919s'). It then listens on the socket for a response.

- `priv` creates a Wireguard interfaces according to the config and configures it. It then moves the wireguard interface into the user namespace of the pid given above. And then writes 'Done' to the socket.

- Then either `client` or `server` configures the Wireguard interface in the network namespace giving it the configured address and setting it up. Then if it is a `client`, it executes moonlight and if `server` executes `sunshine`.
