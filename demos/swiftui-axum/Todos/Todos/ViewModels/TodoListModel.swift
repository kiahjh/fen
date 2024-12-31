import Foundation
import SwiftUI

class TodoListModel: ObservableObject {
  @Published var isLoading = false
  @Published var todos = todosFromDb

  func onFetchTodos() {
    Task {
      self.isLoading = true
      try await Task.sleep(nanoseconds: 1_000_000_000)
      self.todos = todosFromDb
      self.isLoading = false
    }
  }
}

struct Todo: Identifiable {
  var id: UUID
  var name: String
  var description: String?
  var due: Date?
  var isCompleted: Bool
}

let todosFromDb = [
  Todo(
    id: UUID(),
    name: "Buy groceries",
    description: "Milk, eggs, bread, and bananas",
    due: nil,
    isCompleted: false
  ),
  Todo(
    id: UUID(),
    name: "Walk the dog",
    description: "Around the block",
    due: Date(),
    isCompleted: true
  ),
  Todo(
    id: UUID(),
    name: "Do laundry",
    description: "Wash, dry, and fold",
    due: Date(),
    isCompleted: false
  ),
]
