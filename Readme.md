![Test](https://github.com/natir/igusharray/workflows/Test/badge.svg)
![Lints](https://github.com/natir/igusharray/workflows/Lints/badge.svg)
![MSRV](https://github.com/natir/igusharray/workflows/MSRV/badge.svg)
[![CodeCov](https://codecov.io/gh/natir/igusharray/branch/master/graph/badge.svg)](https://codecov.io/gh/natir/igusharray)
[![Documentation](https://github.com/natir/igusharray/workflows/Documentation/badge.svg)](https://natir.github.io/igusharray/igusharray)
[![License](https://img.shields.io/badge/license-MIT-green)](https://github.com/natir/igusharray/blob/master/LICENSE)

# IgushArray

IgushArray could be describ as middle between Array and List.

|Operation   |Array    |IgushArray         |List   |
|:-----------|:--------|:------------------|:------|
|Access      |O (1)    |O (1)              |O (N)  |
|Insert      |O (N)    |**O (N**^**1/2)**  |O (1)  |
|Erase       |O (N)    |**O (N**^**1/2)**  |O (1)  |
|Push Back   |O (1)\*  |O (1)\*            |O (1)  |
|Push Front  |O (N)    |**O (N**^**1/2)**  |O (1)  |

For more information about how work IgushArray you can read original project [readme](https://github.com/igushev/IgushArray).

This is a rust implementation of IgushArray.

## Minimal Support Rust Version

Current minimal support Rust version is 1.60.0
