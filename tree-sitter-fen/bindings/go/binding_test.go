package tree_sitter_fen_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_fen "github.com/kiahjh/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_fen.Language())
	if language == nil {
		t.Errorf("Error loading Fen grammar")
	}
}
