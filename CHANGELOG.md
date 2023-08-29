# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.4.0](https://github.com/szattila98/binge_at_home/compare/0.3.0..0.4.0) - 2023-08-29
### Package updates
- [server-0.2.0](server) bumped to [server-0.2.0](https://github.com/szattila98/binge_at_home/compare/server-0.1.0..server-0.2.0)
### Global changes
#### Features
- **(server)** added catalog crud - ([74d663b](https://github.com/szattila98/binge_at_home/commit/74d663b527797607d5943f845e5685e9986c7a50)) - [@szattila98](https://github.com/szattila98)
- **(server)** added migrate feature that inlines and run migrations - ([2138be0](https://github.com/szattila98/binge_at_home/commit/2138be0d46aa4f7e65f4b6627e350b98d8b8f0db)) - [@szattila98](https://github.com/szattila98)
- **(server)** added migrations with sqlx cli and other necessities - ([d34770b](https://github.com/szattila98/binge_at_home/commit/d34770bf953fdb09a87312ab966f2d886a7fad74)) - [@szattila98](https://github.com/szattila98)
- **(server)** added useful middlewares - ([709ef7b](https://github.com/szattila98/binge_at_home/commit/709ef7bdf9ca8ae05807d1c9a702efc573233442)) - [@szattila98](https://github.com/szattila98)
- **(server)** added init.sql - ([0775dd4](https://github.com/szattila98/binge_at_home/commit/0775dd4fa2d9c343a8ffe1289a1b03808d9fef5d)) - [@szattila98](https://github.com/szattila98)
#### Refactoring
- **(server)** added instrumentation to startup functions - ([1cfe6b7](https://github.com/szattila98/binge_at_home/commit/1cfe6b72955c3e19d3db70011c8dcfb6cef1de95)) - [@szattila98](https://github.com/szattila98)
- **(server)** refactored middleware logic, added config options - ([f549221](https://github.com/szattila98/binge_at_home/commit/f5492219aae16f43e650a34f89b4b003d9495e1f)) - [@szattila98](https://github.com/szattila98)

- - -

## [0.3.0](https://github.com/szattila98/binge_at_home/compare/0.2.0..0.3.0) - 2023-07-18
### Package updates
- [server](server) bumped to [server-0.1.0](https://github.com/szattila98/binge_at_home/compare/2399c6f6a128d954c6644f190614d3fee1f507a4..server-0.1.0)
- [client](client) bumped to [client-0.1.0](https://github.com/szattila98/binge_at_home/compare/2399c6f6a128d954c6644f190614d3fee1f507a4..client-0.1.0)
### Global changes
#### Continuous Integration
- **(client,server)** fixed pushing package tags for ci - ([0cd02ed](https://github.com/szattila98/binge_at_home/commit/0cd02ed1e8e9d2ec16d2e4afe79e94b1d1feffd9)) - [@szattila98](https://github.com/szattila98)

- - -

## [0.2.0](https://github.com/szattila98/binge_at_home/compare/0.1.0..0.2.0) - 2023-07-18
### Package updates
- [server](server) bumped to [server-0.1.0](https://github.com/szattila98/binge_at_home/compare/2399c6f6a128d954c6644f190614d3fee1f507a4..server-0.1.0)
- [client](client) bumped to [client-0.1.0](https://github.com/szattila98/binge_at_home/compare/2399c6f6a128d954c6644f190614d3fee1f507a4..client-0.1.0)
### Global changes
#### Continuous Integration
- **(client)** fixed npm versioning to allow same version if client not changed - ([adf034c](https://github.com/szattila98/binge_at_home/commit/adf034c2ba7b1f16dedfd9951a388d269a5b7704)) - [@szattila98](https://github.com/szattila98)
- removed waiting on client and server checks as they might not run and fail the build - ([76e785b](https://github.com/szattila98/binge_at_home/commit/76e785b07367ef8a9fbd7e203abfa932f5a84066)) - [@szattila98](https://github.com/szattila98)
#### Features
- **(server)** implemented connecting to database - ([0b920a4](https://github.com/szattila98/binge_at_home/commit/0b920a412fe7f801e783729769971fea8c0dfa9e)) - [@szattila98](https://github.com/szattila98)
- **(server)** added postgres database to compose file - ([034a307](https://github.com/szattila98/binge_at_home/commit/034a3077a8fbca4f40d98efb658af467db19aeb5)) - [@szattila98](https://github.com/szattila98)
- **(server)** Application frame added - ([e6298cb](https://github.com/szattila98/binge_at_home/commit/e6298cb365cca854af7a57b5719b0ac3096fde18)) - [@szattila98](https://github.com/szattila98)
#### Miscellaneous Chores
- extended postman collection - ([043ccea](https://github.com/szattila98/binge_at_home/commit/043ccea7102134368eb648a84458bd0902523d43)) - [@doleance](https://github.com/doleance)

- - -

## [0.1.0](https://github.com/szattila98/binge_at_home/compare/2399c6f6a128d954c6644f190614d3fee1f507a4..0.1.0) - 2023-06-30
### Package updates
- [client](client) bumped to [client-0.1.0](https://github.com/szattila98/binge_at_home/compare/2399c6f6a128d954c6644f190614d3fee1f507a4..client-0.1.0)
- [server](server) bumped to [server-0.1.0](https://github.com/szattila98/binge_at_home/compare/2399c6f6a128d954c6644f190614d3fee1f507a4..server-0.1.0)
### Global changes
#### Continuous Integration
- npm ci fix in cog pre-bumb - ([6e0fe6a](https://github.com/szattila98/binge_at_home/commit/6e0fe6a382961c4c1a5ce2381ce634aa0194ca7f)) - [@szattila98](https://github.com/szattila98)
- Release fix - ([2e6f21b](https://github.com/szattila98/binge_at_home/commit/2e6f21bb901c6fda2d865c81124d48e40b5d8d9d)) - [@szattila98](https://github.com/szattila98)
- Wait on check fix - ([02b74cb](https://github.com/szattila98/binge_at_home/commit/02b74cba75e6f6fe89cd6a2cf65c3d4f69c68dff)) - [@szattila98](https://github.com/szattila98)
- Added wait relations between workflows - ([85d65c8](https://github.com/szattila98/binge_at_home/commit/85d65c8011dba548b4f26e6237941200d01d6b90)) - [@szattila98](https://github.com/szattila98)
- Updated actions with better supported dependency actions - ([461b52f](https://github.com/szattila98/binge_at_home/commit/461b52f260430a0a1782489c7394d35318d2b8d8)) - [@szattila98](https://github.com/szattila98)
- added release pipeline - ([249e3c4](https://github.com/szattila98/binge_at_home/commit/249e3c42a85c808f2020fe6ba4d049a1ad9605bc)) - [@szattila98](https://github.com/szattila98)
- cocogitto action setup - ([0fd1f00](https://github.com/szattila98/binge_at_home/commit/0fd1f000fb4db155a72b53bed49add2cef51990a)) - [@szattila98](https://github.com/szattila98)
- added conventional commit check action - ([f007991](https://github.com/szattila98/binge_at_home/commit/f007991b0b880acbfa5b673a786156da25f1ecc3)) - [@szattila98](https://github.com/szattila98)
- basic pipeline setup - ([8096e6b](https://github.com/szattila98/binge_at_home/commit/8096e6b0739278082dd5f5dc00f0c90af1901b08)) - [@szattila98](https://github.com/szattila98)
#### Documentation
- Fixed dev.md - ([d161225](https://github.com/szattila98/binge_at_home/commit/d161225a9b7b583e2b083ca0e11de1f93b3a98bb)) - [@szattila98](https://github.com/szattila98)
- Completed DEV.md - ([8e3ecb1](https://github.com/szattila98/binge_at_home/commit/8e3ecb1861ef494b207462c5332abbb208d781f9)) - [@szattila98](https://github.com/szattila98)
- Added README.md and WIP DEV.md - ([e9f8219](https://github.com/szattila98/binge_at_home/commit/e9f82193e42213d831e841d3d61bfd051d533c48)) - [@szattila98](https://github.com/szattila98)
#### Miscellaneous Chores
- automatic git line ending config - ([d082b60](https://github.com/szattila98/binge_at_home/commit/d082b60cbf62103b410ca9216f80c29371527dca)) - [@szattila98](https://github.com/szattila98)
- added windows specific prettier rule - ([d8dc366](https://github.com/szattila98/binge_at_home/commit/d8dc366418de05adbcdae1eb777d2b052b5bca04)) - [@doleance](https://github.com/doleance)
- Moved around dockerfiles and made some client optimizations with dumb-init - ([51ac863](https://github.com/szattila98/binge_at_home/commit/51ac863d4424ad5b3d798e6add0443a158407b05)) - [@szattila98](https://github.com/szattila98)
- rewrote justfile, added cog hook to config - ([9eb54ac](https://github.com/szattila98/binge_at_home/commit/9eb54ac6c7428b704e47ac0df0789bdb5449d59e)) - [@szattila98](https://github.com/szattila98)
- deleted empty changelog - ([3b69069](https://github.com/szattila98/binge_at_home/commit/3b69069523ea56a986c7f9c4f1a8a9db53c45239)) - [@szattila98](https://github.com/szattila98)
- dockerization - ([c686f53](https://github.com/szattila98/binge_at_home/commit/c686f53bf3dcf2dd2d649347888641011dab2220)) - [@szattila98](https://github.com/szattila98)
- added justfile with tooling checks - ([22ec01c](https://github.com/szattila98/binge_at_home/commit/22ec01cda97133a65dac69e2913715a33dde1046)) - [@szattila98](https://github.com/szattila98)
#### Style
- **(server)** Added serious lint command to justfile - ([ecc3c75](https://github.com/szattila98/binge_at_home/commit/ecc3c750a231d969b12a751fee2bdf07acb8a5b6)) - [@szattila98](https://github.com/szattila98)

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).