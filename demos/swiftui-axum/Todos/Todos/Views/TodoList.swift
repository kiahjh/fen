import SwiftUI

struct TodoList: View {
  @StateObject private var viewModel = TodoListModel()

  var body: some View {
    if self.viewModel.todos.isEmpty {
      VStack(spacing: 20) {
        Text("Todos")
        Button(action: self.viewModel.onFetchTodos) {
          Text(self.viewModel.isLoading ? "Loading..." : "Fetch Todos")
            .foregroundStyle(.white)
            .padding(8)
            .background(self.viewModel.isLoading ? .gray : .blue)
            .font(.system(size: 20, weight: .bold))
            .cornerRadius(8)
        }
      }
    } else {
      VStack {
        Spacer()
        VStack(spacing: 12) {
          ForEach(self.viewModel.todos) { todo in
            TodoCard(todo: todo)
          }
        }
        Spacer()
      }
      .padding(20)
      .background(.gray.opacity(0.05))
    }
  }
}

#Preview {
  TodoList()
}
