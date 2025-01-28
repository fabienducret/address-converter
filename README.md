# Address Converter

This project is a CLI that converts addresses :

- from french format to structured ISO 20022
- from structured ISO 20022 to french format

The address is then stored in a database.

Here is the link to the [related
documentation](https://www.cfonb.org/fichiers/20210621111227_Guide_CFONB_Recommandations_Transposition_en_adresse_postale_structuree_ISO__V1.0.pdf).

## Usage

### From french to structured ISO 20022

Example of a conversion from french to structured ISO 20022.

#### Command

```bash 
cargo run -- --address-kind french --line1 "nom adresse" --line2 "Mademoiselle Lucie MARTIN" --line3 "Résidence des Capucins Bâtiment Quater" --line4 "56 RUE EMILE ZOLA" --line5 "BP 90432 MONTFERRIER SUR LEZ" --line6 "34092 MONTPELLIER CEDEX 5"  --line7 France
```

#### Result

```bash
StructuredAddress { department: "Mademoiselle Lucie MARTIN", sub_department: "", building_name: "", floor: "Résidence des Capucins Bâtiment Quater", room: "", street_name: "RUE EMILE ZOLA", building_number: "56", post_box: "BP 90432", town_location_name: "MONTFERRIER SUR LEZ", post_code: "34092", town_name: "MONTPELLIER CEDEX 5", country: "FR", district_name: "", country_sub_division: "" }
```

### From structured ISO 20022 to french

Example of a conversion from structured ISO 20022 to french.

#### Command

```bash 
cargo run -- --address-kind structured --department "Service achat" --street-name "22BIS RUE DES FLEURS" --floor "Zone industrielle de la Ballastrierre" --post-box "BP 40122" --post-code "33506" --town-name "LIBOURNE CEDEX"  --country "FR"
```

#### Result

```bash
FrenchAddress { line_1: "", line_2: "Service achat", line_3: "Zone industrielle de la Ballastrierre", line_4: "22BIS RUE DES FLEURS", line_5: "BP 40122", line_6: "33506 LIBOURNE CEDEX", line_7: "France" }
```

## Tests

This command starts both unit and integration tests.

```bash
cargo test 
```

### Unit tests

These tests are directly located in the `src` folder.

### Integration tests

These tests are located in the `tests` folder.

## Technical choices

- use of `ports and adapters` pattern to have a decoupled application (the domain is isolated from the infra)
- use of manual functions composition (no DI container)
- injection of the `uuid generator` to keep a pure domain with determinist tests
- use of `closures` for the `usecases` : the `usecase` represents a unique action so we don't need the boilerplate of a
  `struct`
- use of `github actions` for CI build

## Env

The `DB_FILE` environment variable is used to configure the path of the file where the addresses are stored.
It can be set like this :

```bash
DB_FILE="./path/to/my/db.json" cargo run -- --address-kind structured --department "Service achat" --street-name "22BIS RUE DES FLEURS" --floor "Zone industrielle de la Ballastrierre" --post-box "BP 40122" --post-code "33506" --town-name "LIBOURNE CEDEX"  --country "FR"
```

The default value `./db.json` is used if the environment variable is not provided.

## Improvements

- [ ] Validate arguments sent to the CLI
- [ ] Add more tests on address conversion
