import XCTest
import SwiftTreeSitter
import TreeSitterFen

final class TreeSitterFenTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_fen())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Fen grammar")
    }
}
