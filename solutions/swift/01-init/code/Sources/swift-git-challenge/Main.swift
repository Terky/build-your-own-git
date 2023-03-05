import Foundation

@main
public struct Main {
    public static func main() throws {
        guard CommandLine.argc >= 2 else {
            print("usage: mygit <command> [<args>...]")
            return
        }

        let command = CommandLine.arguments[1]
        switch command {
        case "init":
            let fileManager = FileManager.default

            try fileManager.createDirectory(atPath: ".git", withIntermediateDirectories: false)
            try fileManager.createDirectory(atPath: ".git/objects", withIntermediateDirectories: false)
            try fileManager.createDirectory(atPath: ".git/refs", withIntermediateDirectories: false)
            fileManager.createFile(atPath: ".git/HEAD", contents: "ref: refs/heads/master\n".data(using: .utf8))

            print("Initialized git directory")
        default:
            print("Unknown command \(command)")
            return
        }
    }
}