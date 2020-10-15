# fns(1) - fns - Find NordVPN Server

# SYNOPSIS

```
fns 1.0
Spudmash Media [ - ]
Reverse Lookup of NordVPN Server hostname by Ipv4 address
Built with Rust 🦀

USAGE:
    fns [FLAGS] [OPTIONS] --country <Country Code> --ip <Ip4 Address>

FLAGS:
    -v, --verbose    Verbose mode will print out CPU information & suggestions
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --country <Country Code>        Options: [Al, Ar, Au, At, Be, Ba, Br, Bg, Ca, Cl, Cr, 
                                        Hr, Cy, Cz, Dk, Ee, Fi, Fr, Ge, De, Gr, Hk, Hu, Is, In,
                                        Id, Ie, Il, It, Jp, Lv, Lu, My, Mx, Md, Nl, Nz, Mk, No,
                                        Pl, Pt, Ro, Rs, Sg, Sk, Si, Za, Kr, Es, Se, Ch, Tw, Th,
                                        Tr, Ua, Uk, Us, Vn] [default: Au]
    -e, --end <End Number>              Default: 1000
    -i, --ip <Ip4 Address>              Search for VPN Hostname by IP address. E.g. 127.0.0.1
    -s, --start <Start Number>          Default: 1
    -t, --threadcount <Thread Count>    Thread Count [Default to number of physical CPU cores]
```

# USAGE

![demo](/docs/img/fns-demo.gif)

## Simple
```
fns-cli> ./fns -c au -i 100.0.0.1

Where
c = country code
i = ip address to search
```

## Full
```
fns-cli> ./fns -c au -i 100.0.0.1 -s 1 -e 1000 -v -t 32

Where
c = country code
i = ip address to search
s = start index
e = end index
t = thread count
```


## Verbose - Show CPU/Thread information
```
fns-cli> ./fns -c au -i 100.0.0.1 -v
```
Use **-v** option to have the application print out useful information about your CPU and available threads

```
::CPU Information::
 - 4 cores available
 - 8 threads available

💡 Optimize search speed by doubling the thread count or higher. E.g. -t 16

🥞 8 Threads requested. Distributing workload...
```

# Getting Started
- [Prequisite Before Starting](/docs/prerequisite.md)
- [Build Instructions](/docs/build.md)
- [Testing Instructions](/docs/testing.md)

# License
This code is distributed under the terms and conditions of the [MIT License](/LICENSE)
