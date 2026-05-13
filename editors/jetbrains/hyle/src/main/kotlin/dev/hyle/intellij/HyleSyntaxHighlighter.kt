package dev.hyle.intellij

import com.intellij.lexer.Lexer
import com.intellij.openapi.editor.DefaultLanguageHighlighterColors
import com.intellij.openapi.editor.HighlighterColors
import com.intellij.openapi.editor.colors.TextAttributesKey
import com.intellij.openapi.fileTypes.SyntaxHighlighter
import com.intellij.psi.TokenType
import com.intellij.psi.tree.IElementType

class HyleSyntaxHighlighter : SyntaxHighlighter {
    override fun getHighlightingLexer(): Lexer = HyleLexer()

    override fun getTokenHighlights(tokenType: IElementType): Array<TextAttributesKey> =
        when (tokenType) {
            HyleTokenTypes.COMMENT -> COMMENT_KEYS
            HyleTokenTypes.DIRECTIVE -> DIRECTIVE_KEYS
            HyleTokenTypes.KEYWORD -> KEYWORD_KEYS
            HyleTokenTypes.TYPE -> TYPE_KEYS
            HyleTokenTypes.CONSTANT -> CONSTANT_KEYS
            HyleTokenTypes.NUMBER -> NUMBER_KEYS
            HyleTokenTypes.STRING -> STRING_KEYS
            HyleTokenTypes.OPERATOR -> OPERATOR_KEYS
            TokenType.BAD_CHARACTER -> BAD_CHAR_KEYS
            else -> EMPTY_KEYS
        }

    companion object {
        val COMMENT = TextAttributesKey.createTextAttributesKey(
            "HYLE_COMMENT",
            DefaultLanguageHighlighterColors.LINE_COMMENT
        )

        val DIRECTIVE = TextAttributesKey.createTextAttributesKey(
            "HYLE_DIRECTIVE",
            DefaultLanguageHighlighterColors.METADATA
        )

        val KEYWORD = TextAttributesKey.createTextAttributesKey(
            "HYLE_KEYWORD",
            DefaultLanguageHighlighterColors.KEYWORD
        )

        val TYPE = TextAttributesKey.createTextAttributesKey(
            "HYLE_TYPE",
            DefaultLanguageHighlighterColors.CLASS_NAME
        )

        val CONSTANT = TextAttributesKey.createTextAttributesKey(
            "HYLE_CONSTANT",
            DefaultLanguageHighlighterColors.CONSTANT
        )

        val NUMBER = TextAttributesKey.createTextAttributesKey(
            "HYLE_NUMBER",
            DefaultLanguageHighlighterColors.NUMBER
        )

        val STRING = TextAttributesKey.createTextAttributesKey(
            "HYLE_STRING",
            DefaultLanguageHighlighterColors.STRING
        )

        val OPERATOR = TextAttributesKey.createTextAttributesKey(
            "HYLE_OPERATOR",
            DefaultLanguageHighlighterColors.OPERATION_SIGN
        )

        val BAD_CHARACTER = TextAttributesKey.createTextAttributesKey(
            "HYLE_BAD_CHARACTER",
            HighlighterColors.BAD_CHARACTER
        )

        private val COMMENT_KEYS = arrayOf(COMMENT)
        private val DIRECTIVE_KEYS = arrayOf(DIRECTIVE)
        private val KEYWORD_KEYS = arrayOf(KEYWORD)
        private val TYPE_KEYS = arrayOf(TYPE)
        private val CONSTANT_KEYS = arrayOf(CONSTANT)
        private val NUMBER_KEYS = arrayOf(NUMBER)
        private val STRING_KEYS = arrayOf(STRING)
        private val OPERATOR_KEYS = arrayOf(OPERATOR)
        private val BAD_CHAR_KEYS = arrayOf(BAD_CHARACTER)
        private val EMPTY_KEYS = emptyArray<TextAttributesKey>()
    }
}
