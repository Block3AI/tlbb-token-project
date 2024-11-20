/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/tlbb_token.json`.
 */
export type TlbbToken = {
  "address": "Euueq2HBsDG4Dwf1QtHRrXCSnmdUQmBXJHqfsWDhYYX1",
  "metadata": {
    "name": "tlbbToken",
    "version": "0.1.0",
    "spec": "0.1.0"
  },
  "instructions": [
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "myAccount",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "myData",
      "discriminator": [
        145,
        158,
        1,
        176,
        150,
        99,
        252,
        226
      ]
    }
  ],
  "types": [
    {
      "name": "myData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "value",
            "type": "u64"
          }
        ]
      }
    }
  ]
};
