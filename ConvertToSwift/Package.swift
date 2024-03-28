// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: ConvertToSwift

import PackageDescription;

let package = Package(
    name: "ConvertToSwift",
    platforms: [
        .iOS(.v13),
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "ConvertToSwift",
            targets: ["ConvertToSwift"]
        )
    ],
    dependencies: [ ],
    targets: [
        .binaryTarget(name: "RustFramework", path: "./RustFramework.xcframework"),
        .target(
            name: "ConvertToSwift",
            dependencies: [
                .target(name: "RustFramework")
            ]
        ),
    ]
)