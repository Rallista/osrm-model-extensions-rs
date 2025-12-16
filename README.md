# osrm-extensions-rs

Rust library for extending OSRM route responses for modern navigation SDKs like 
[ferrostar](https://github.com/Stadiamaps/ferrostar).

## Usage

This library processes and extends standardized JSON models from OSRM routing servers. It can be used 
in a rust based proxy server that calls an OSRM API, such as [valhalla](https://github.com/valhalla/valhalla) with OSRM format. It's goal is to enhance the response json using data provided by the OSRM server's response.

It includes features like banner instructions, voice instructions and max speed (speed limit) and 
other features commonly used by modern navigation SDKs like 
[ferrostar](https://github.com/Stadiamaps/ferrostar).

## Contributing

This project has several opportunities for extension and improvement. The foundation is there, but 
many of the fancier features you see in professional navigation apps like Google Maps are not yet 
implemented.

This library includes [rust_i18n](https://docs.rs/rust-i18n/latest/rust_i18n) for localization. Currently, 
it only supports basic english. Since we're only extending localized responses from routing servers, 
the scope of what needs translation is pretty limited and may eventually be better handled by an external
rust crate.

### The Future

This system is limited by the scope of data provided by the OSRM server's response. As a result, it's 
likely that this library will eventually be superseded by a library that works off of the richer context
held by a library like [valinor](https://github.com/Stadiamaps/valinor).

## Deeper Dive

### Route Step Example

It's common for OSRM route servers (e.g. Valhalla with OSRM format) to return route steps that look like this:

```json
{
    "intersections": [
        {
            "bearings": [
                90
            ],
            "entry": [
                true
            ],
            "admin_index": 0,
            "out": 0,
            "geometry_index": 0,
            "location": [
                -108.365587,
                39.115856
            ]
        }
    ],
    "speedLimitUnit": "mph",
    "maneuver": {
        "type": "depart",
        "instruction": "Drive east on G 7/10 Road/G.7.",
        "bearing_after": 90,
        "bearing_before": 0,
        "location": [
            -108.365587,
            39.115856
        ]
    },
    "speedLimitSign": "mutcd",
    "name": "G 7/10 Road",
    "duration": 49.121,
    "distance": 477.565,
    "driving_side": "right",
    "weight": 45.437,
    "mode": "driving",
    "ref": "G.7",
    "geometry": "_dmriAdpbumEP{|El@_NfCuTjEge@|Ecm@"
},
```

But many navigation SDK's require some additional information for basic functionality. 
Here's the same step after being processed and modified by this library:

```json
{
    "distance": 477.565,
    "duration": 49.121,
    "geometry": "_dmriAdpbumEP{|El@_NfCuTjEge@|Ecm@",
    "weight": 45.437,
    "name": "G 7/10 Road",
    "ref": "G.7",
    "mode": "driving",
    "maneuver": {
        "location": [
            -108.365587,
            39.115856
        ],
        "bearing_before": 0,
        "bearing_after": 90,
        "type": "depart",
        "instruction": "Drive east on G 7/10 Road/G.7."
    },
    "intersections": [
        {
            "location": [
                -108.365587,
                39.115856
            ],
            "bearings": [
                90
            ],
            "entry": [
                true
            ],
            "out": 0
        }
    ],
    "driving_side": "right",
    "voiceInstructions": [
        {
            "distanceAlongGeometry": 468.0137,
            "announcement": "Drive east on G 7/10 Road/G.7."
        },
        {
            "distanceAlongGeometry": 70.0,
            "announcement": "Turn left onto Elberta Avenue."
        }
    ],
    "bannerInstructions": [
        {
            "distanceAlongGeometry": 477.565,
            "primary": {
            "text": "Elberta Avenue",
            "type": "turn",
            "modifier": "left",
            "components": [
                {
                "text": "Elberta Avenue",
                "type": "text"
                }
            ]
            }
        }
    ]
},
```

### Voice Instructions

Voice instructions are used by navigation SDKs to announce instructions along the traversal of a step.
A step's voice instructions can range from an extermely small step scenario where like
"Turn right on Alberta Avenue, then turn left on Elberta Street." All the way to more 
complex scenarios where a long high speed step has 4 instructions: 

```json
[
  {
    "distanceAlongGeometry": 7116.76,
    "announcement": "Continue on I 70 for four miles"
  },
  {
    "distanceAlongGeometry": 1609.344,
    "announcement": "In one mile, take exit 37 onto i 70 business loop/us 6/us 50 toward clifton/grand junction/delta."
  },
  {
    "distanceAlongGeometry": 804.672,
    "announcement": "In one half mile, take exit 37 onto i 70 business loop/us 6/us 50 toward clifton/grand junction/delta."
  },
  {
    "distanceAlongGeometry": 150.0,
    "announcement": "Take exit 37 onto I 70 Business Loop/US 6/US 50 toward Clifton/Grand Junction/Delta."
  }
]
```

To handle these scenarios, this works with the steps, geometry, adjacent steps and formulates 
the best combination of instructions. There's plenty of room for improvement here, but it's 
a good start.

## References

- [CONTRIBUTING.md](CONTRIBUTING.md)
- [OSRM](https://github.com/Project-OSRM/osrm-backend)
- [Valhalla](https://github.com/valhalla/valhalla)
- Feature inspiration - [Mapbox Directions API](https://docs.mapbox.com/api/navigation/directions/)
