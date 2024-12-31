import SwiftUI

struct TodoCard: View {
  var todo: Todo

  var body: some View {
    HStack(alignment: .top, spacing: 12) {
      Rectangle()
        .frame(width: 20, height: 20)
        .foregroundStyle(self.todo.isCompleted ? .blue : .white)
        .cornerRadius(6)
        .overlay {
          RoundedRectangle(cornerRadius: 6)
            .stroke(self.todo.isCompleted ? .blue : .gray.opacity(0.5), lineWidth: 2)
          Image(systemName: "checkmark")
            .foregroundStyle(.white)
            .font(.system(size: 12, weight: .bold))
        }
        .offset(y: 4)
      VStack(alignment: .leading) {
        Text(self.todo.name)
          .font(.system(size: 24, weight: .semibold))
        if let description = todo.description {
          Text(description)
            .foregroundStyle(.secondary)
        }
        if let due = todo.due {
          Text("Due: \(due, style: .date)")
            .font(.body)
            .foregroundStyle(.red)
            .padding(.top, 4)
        }
      }
      Spacer()
    }
    .frame(maxWidth: .infinity)
    .padding(16)
    .background(.white)
    .cornerRadius(10)
    .shadow(radius: 5)
  }
}
