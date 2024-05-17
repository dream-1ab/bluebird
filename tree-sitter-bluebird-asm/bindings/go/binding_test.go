package tree_sitter_bluebird_asm_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-bluebird_asm"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_bluebird_asm.Language())
	if language == nil {
		t.Errorf("Error loading BluebirdAsm grammar")
	}
}
