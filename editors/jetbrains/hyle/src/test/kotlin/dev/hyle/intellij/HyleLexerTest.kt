package dev.hyle.intellij

import com.intellij.psi.TokenType
import com.intellij.psi.tree.IElementType
import org.junit.Assert.assertEquals
import org.junit.Test

class HyleLexerTest {
    @Test
    fun `lexes core hyle tokens`() {
        val tokens = lex("""#hyle 0.1
model Fire {
  next Fire.intensity = "hot" // comment
}
@""")

        assertEquals(
            listOf(
                HyleTokenTypes.DIRECTIVE to "#hyle",
                HyleTokenTypes.NUMBER to "0.1",
                HyleTokenTypes.KEYWORD to "model",
                HyleTokenTypes.IDENTIFIER to "Fire",
                HyleTokenTypes.OPERATOR to "{",
                HyleTokenTypes.KEYWORD to "next",
                HyleTokenTypes.IDENTIFIER to "Fire",
                HyleTokenTypes.OPERATOR to ".",
                HyleTokenTypes.IDENTIFIER to "intensity",
                HyleTokenTypes.OPERATOR to "=",
                HyleTokenTypes.STRING to "\"hot\"",
                HyleTokenTypes.COMMENT to "// comment",
                HyleTokenTypes.OPERATOR to "}",
                TokenType.BAD_CHARACTER to "@",
            ),
            tokens.filterNot { it.first == TokenType.WHITE_SPACE }
        )
    }

    private fun lex(text: String): List<Pair<IElementType, String>> {
        val lexer = HyleLexer()
        lexer.start(text)

        val tokens = mutableListOf<Pair<IElementType, String>>()
        while (lexer.tokenType != null) {
            val tokenType = lexer.tokenType ?: break
            val tokenText = text.substring(lexer.tokenStart, lexer.tokenEnd)
            tokens += tokenType to tokenText
            lexer.advance()
        }

        return tokens
    }
}
