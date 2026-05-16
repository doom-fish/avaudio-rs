// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "AVAudioBridge",
    platforms: [
        .macOS(.v12)
    ],
    products: [
        .library(name: "AVAudioBridge", type: .static, targets: ["AVAudioBridge"])
    ],
    targets: [
        .target(
            name: "AVAudioBridge",
            dependencies: [],
            path: "Sources/AVAudioBridge",
            publicHeadersPath: "include"
        )
    ]
)
