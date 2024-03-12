# minecraft-formats
 A collection of formats made for minecraft mods, such as [sc-peripherals' 3dj](https://docs.sc3.io/features/sc-peripherals.html#_3dj-format) and custom ones for other mods

## Formats

### `.3dj`
A format that was specifically made for [sc-peripherals](https://modrinth.com/mod/sc-peripherals) to be used with their 3D Printers.

Rust structs for this format can be found [here](./src/formats/printer.rs), documentation for this format can be found [here](https://docs.sc3.io/features/sc-peripherals.html#_3dj-format)

### `.statue`
A custom format made for [UnlimitedPeripheralWorks](https://modrinth.com/mod/unlimitedperipheralworks)' [Statue Workbench](https://docs.siredvin.site/UnlimitedPeripheralWorks/peripherals/statue_workbench/).

Rust structs for this format can be found [here](./src/formats/statue.rs)