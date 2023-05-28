# distro-watch

Fetch a distro's metadata through [DistroWatch](https://distrowatch.com/), with a
single function call:

```rust
let fedora = stat("fedora").await.unwrap().unwrap();
println!("{:?}", fedora);
```

```shell
$ cargo r -q
DistroMetadata { name: "fedora", based_on: [], origin: "USA", architecture: ["aarch64", "ppc64le", "s390x", "x86_64"], desktop: ["Awesome", "Budgie", "Cinnamon", "Enlightenment", "i3", "GNOME", "KDE Plasma", "LXDE", "LXQt", "MATE", "Openbox", "Pantheon", "Ratpoison", "Xfce"], category: ["Desktop", "Immutable", "Server", "Live Medium"], status: Active, popularity: "6 (1,178 hits per day)" }
```