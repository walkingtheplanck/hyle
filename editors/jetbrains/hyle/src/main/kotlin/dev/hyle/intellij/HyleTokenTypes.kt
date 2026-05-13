package dev.hyle.intellij

import com.intellij.psi.tree.IElementType

class HyleTokenType(debugName: String) : IElementType(debugName, HyleLanguage)

object HyleTokenTypes {
    val COMMENT = HyleTokenType("COMMENT")
    val DIRECTIVE = HyleTokenType("DIRECTIVE")
    val KEYWORD = HyleTokenType("KEYWORD")
    val TYPE = HyleTokenType("TYPE")
    val CONSTANT = HyleTokenType("CONSTANT")
    val NUMBER = HyleTokenType("NUMBER")
    val IDENTIFIER = HyleTokenType("IDENTIFIER")
    val OPERATOR = HyleTokenType("OPERATOR")
    val STRING = HyleTokenType("STRING")
    val BAD_CHARACTER = HyleTokenType("BAD_CHARACTER")
}
