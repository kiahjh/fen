import Foundation
import Testing

@testable import SwiftClient

@Test func ints() throws {
    // input
    let input = 42
    let inputJson = try APIClient.encodeAsString(input)
    #expect(inputJson == "42")

    // output
    let outputJson = """
        {
          "data": 42
        }
        """
    let output = try APIClient.decode(outputJson, type: SuccessResponse<Int>.self)
    #expect(output.data == 42)
}

@Test func strings() throws {
    // input
    let input = "hello"
    let inputJson = try APIClient.encodeAsString(input)
    #expect(inputJson == "\"hello\"")

    // output
    let outputJson = """
        {
          "data": "hello"
        }
        """
    let output = try APIClient.decode(outputJson, type: SuccessResponse<String>.self)
    #expect(output.data == "hello")
}

@Test func bools() throws {
    // input
    let input = true
    let inputJson = try APIClient.encodeAsString(input)
    #expect(inputJson == "true")

    // output
    let outputJson = """
        {
          "data": true
        }
        """
    let output = try APIClient.decode(outputJson, type: SuccessResponse<Bool>.self)
    #expect(output.data == true)
}

@Test func floats() throws {
    // input
    let input = 3.14
    let inputJson = try APIClient.encodeAsString(input)
    #expect(inputJson == "3.14")

    // output
    let outputJson = """
        {
          "data": 3.14
        }
        """
    let output = try APIClient.decode(outputJson, type: SuccessResponse<Double>.self)
    #expect(output.data == 3.14)
}

@Test func uuids() throws {
    // input
    let input = UUID(uuidString: "00000000-0000-0000-0000-000000000000")!
    let inputJson = try APIClient.encodeAsString(input)
    #expect(inputJson == "\"00000000-0000-0000-0000-000000000000\"")

    // output
    let outputJson = """
        {
          "data": "00000000-0000-0000-0000-000000000000"
        }
        """
    let output = try APIClient.decode(outputJson, type: SuccessResponse<UUID>.self)
    #expect(output.data == UUID(uuidString: "00000000-0000-0000-0000-000000000000"))
}

@Test func dates() throws {
    // input
    let input = Date(timeIntervalSince1970: 0)
    let inputJson = try APIClient.encodeAsString(input)
    #expect(inputJson == "\"1970-01-01T00:00:00Z\"")

    // output
    let outputJson = """
        {
          "data": "1970-01-01T00:00:00Z"
        }
        """
    let output = try APIClient.decode(outputJson, type: SuccessResponse<Date>.self)
    #expect(output.data == Date(timeIntervalSince1970: 0))
}

@Test func arrays() throws {
    // input
    let input = [1, 2, 3]
    let inputJson = try APIClient.encodeAsString(input)
    #expect(inputJson == "[1,2,3]")

    // output
    let outputJson = """
        {
          "data": ["00000000-0000-0000-0000-000000000000", "00000000-0000-0000-0000-000000000001", "00000000-0000-0000-0000-000000000002"]
        }
        """
    let output = try APIClient.decode(outputJson, type: SuccessResponse<[UUID]>.self)
    #expect(
        output.data == [
            UUID(uuidString: "00000000-0000-0000-0000-000000000000"),
            UUID(uuidString: "00000000-0000-0000-0000-000000000001"),
            UUID(uuidString: "00000000-0000-0000-0000-000000000002"),
        ]
    )
}

@Test func optionals() throws {
    // input
    let input1: Int? = 42
    let inputJson1 = try APIClient.encodeAsString(input1)
    #expect(inputJson1 == "42")

    let input2: Int? = nil
    let inputJson2 = try APIClient.encodeAsString(input2)
    #expect(inputJson2 == "null")

    // output
    let outputJson1 = """
        {
          "data": "1970-01-01T00:00:00Z"
        }
        """
    let output1 = try APIClient.decode(outputJson1, type: SuccessResponse<Date?>.self)
    #expect(output1.data == Date(timeIntervalSince1970: 0))

    let outputJson2 = """
        {
          "data": null
        }
        """
    let output2 = try APIClient.decode(outputJson2, type: SuccessResponse<Date?>.self)
    #expect(output2.data == nil)
}

@Test func compoundArraysAndOptionals() throws {
    // input
    let input: [[Int]?] = [[1, 2, 3], nil, [4, 5, 6]]
    let inputJson = try APIClient.encodeAsString(input)
    #expect(inputJson == "[[1,2,3],null,[4,5,6]]")

    // output
    let outputJson1 = """
        {
          "data": ["00000000-0000-0000-0000-000000000000", "00000000-0000-0000-0000-000000000001", "00000000-0000-0000-0000-000000000002"]
        }
        """
    let output1 = try APIClient.decode(outputJson1, type: SuccessResponse<[UUID]?>.self)
    #expect(
        output1.data! == [
            UUID(uuidString: "00000000-0000-0000-0000-000000000000"),
            UUID(uuidString: "00000000-0000-0000-0000-000000000001"),
            UUID(uuidString: "00000000-0000-0000-0000-000000000002"),
        ]
    )

    let outputJson2 = """
        {
          "data": null
        }
        """
    let output2 = try APIClient.decode(outputJson2, type: SuccessResponse<[UUID]?>.self)
    #expect(output2.data == nil)
}

@Test func basicStructs() throws {
    // input
    let input = BasicStructsTestInput(
        id: UUID(uuidString: "00000000-0000-0000-0000-000000000000")!)
    let inputJson = try APIClient.encodeAsString(input)
    #expect(inputJson == "{\"id\":\"00000000-0000-0000-0000-000000000000\"}")

    // output
    let outputJson = """
        {
          "data": {
            "name": "Alice",
            "age": 42,
            "birthday": "1970-01-01T00:00:00Z",
            "hasBeard": false
          }
        }
        """
    let output = try APIClient.decode(
        outputJson, type: SuccessResponse<BasicStructsTestOutput>.self)
    #expect(
        output.data
            == BasicStructsTestOutput(
                name: "Alice",
                age: 42,
                birthday: Date(timeIntervalSince1970: 0),
                hasBeard: false
            )
    )
}

@Test func structsWithCompoundTypes() throws {
    // input
    let input1 = StructsWithCompoundTypesTestInput(foo: "bar")
    let inputJson1 = try APIClient.encodeAsString(input1)
    #expect(inputJson1 == "{\"foo\":\"bar\"}")

    let input2 = StructsWithCompoundTypesTestInput(foo: nil)
    let inputJson2 = try APIClient.encodeAsString(input2)
    // #expect(inputJson2 == "{\"foo\":null}")

    // output
    let outputJson = """
        {
          "data": {
            "bar": [3, null, 1]
          }
        }
        """
    let output = try APIClient.decode(
        outputJson, type: SuccessResponse<StructsWithCompoundTypesTestOutput>.self
    )
    #expect(
        output.data
            == StructsWithCompoundTypesTestOutput(
                bar: [3, nil, 1]
            )
    )
}

@Test func nestedStructs() throws {
    // no input

    // output
    let outputJson = """
        {
          "data": {
            "name": "Alice",
            "birthday": "1970-01-01T00:00:00Z",
            "vehicle": {
              "color": "red",
              "year": 2000
            }
          }
        }
        """
    let output = try APIClient.decode(
        outputJson, type: SuccessResponse<Human>.self
    )
    #expect(
        output.data
            == Human(
                name: "Alice",
                birthday: Date(timeIntervalSince1970: 0),
                vehicle: Vehicle(color: "red", year: 2000)
            )
    )
}

@Test func basicEnums() throws {
    // input
    let input1 = BasicEnumsTestInput.happy
    let inputJson1 = try APIClient.encodeAsString(input1)
    #expect(inputJson1 == "{\"type\":\"happy\"}")

    let input2 = BasicEnumsTestInput.sad
    let inputJson2 = try APIClient.encodeAsString(input2)
    #expect(inputJson2 == "{\"type\":\"sad\"}")

    // output
    let outputJson1 = """
        {
          "data": {
            "type": "foo"
          }
        }
        """
    let output1 = try APIClient.decode(
        outputJson1, type: SuccessResponse<BasicEnumsTestOutput>.self
    )
    #expect(output1.data == BasicEnumsTestOutput.foo)

    let outputJson2 = """
        {
          "data": {
            "type": "bar"
          }
        }
        """
    let output2 = try APIClient.decode(
        outputJson2, type: SuccessResponse<BasicEnumsTestOutput>.self
    )
    #expect(output2.data == BasicEnumsTestOutput.bar)
}

@Test func enumsWithAssociatedValues() throws {
    // no input

    // output
    let outputJson1 = """
        {
          "data": {
            "type": "firstOption",
            "value": 42
          }
        }
        """
    let output1 = try APIClient.decode(
        outputJson1, type: SuccessResponse<EnumsWithAssociatedValuesTestOutput>.self
    )
    #expect(output1.data == .firstOption(42))

    let outputJson2 = """
        {
          "data": {
            "type": "secondOption",
            "value": ["hello", "world"]
          }
        }
        """
    let output2 = try APIClient.decode(
        outputJson2, type: SuccessResponse<EnumsWithAssociatedValuesTestOutput>.self
    )
    #expect(output2.data == .secondOption(["hello", "world"]))
}

@Test func composingStructsAndEnums() throws {
    // no input

    // output
    let outputJson1 = """
        {
          "data": {
            "name": "Alice",
            "birthday": "1970-01-01T00:00:00Z",
            "id": "00000000-0000-0000-0000-000000000000",
            "car": {
              "color": "red",
              "gear": {
                "type": "park"
              }
            }
          }
        }
        """
    let output1 = try APIClient.decode(outputJson1, type: SuccessResponse<Person>.self)
    #expect(
        output1.data
            == Person(
                name: "Alice",
                birthday: Date(timeIntervalSince1970: 0),
                id: UUID(uuidString: "00000000-0000-0000-0000-000000000000")!,
                car: Car(color: "red", gear: .park)
            )
    )

    let outputJson2 = """
        {
          "data": {
            "name": "Bob",
            "birthday": "1970-01-01T00:00:00Z",
            "id": "00000000-0000-0000-0000-000000000001",
            "car": {
              "color": "blue",
              "gear": {
                "type": "drive",
                "value": {
                  "type": "fifth"
                }
              }
            }
          }
        }
        """
    let output2 = try APIClient.decode(outputJson2, type: SuccessResponse<Person>.self)
    #expect(
        output2.data
            == Person(
                name: "Bob",
                birthday: Date(timeIntervalSince1970: 0),
                id: UUID(uuidString: "00000000-0000-0000-0000-000000000001")!,
                car: Car(color: "blue", gear: .drive(.fifth))
            )
    )
}
