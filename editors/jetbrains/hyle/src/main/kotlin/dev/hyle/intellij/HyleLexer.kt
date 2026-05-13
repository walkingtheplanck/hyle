package dev.hyle.intellij

import com.intellij.lexer.LexerBase
import com.intellij.psi.TokenType
import com.intellij.psi.tree.IElementType

class HyleLexer : LexerBase() {
    private var buffer: CharSequence = ""
    private var endOffset = 0
    private var tokenStart = 0
    private var tokenEnd = 0
    private var tokenType: IElementType? = null

    private val keywords = setOf(
        "model", "fields", "neighborhood", "range", "in", "let", "next", "when", "sum", "neighbors"
    )

    private val types = setOf("Int", "Float", "Bool")

    private val constants = setOf(
        "true", "false",
        "Line", "Triangle", "Square", "Hexagon", "Cube", "Tetrahedron",
        "TruncatedOctahedron", "RhombicDodecahedron", "Tesseract",
        "Manhattan", "Euclidean", "Chebyshev",
        "Average", "Nearest", "Sum", "All"
    )

    override fun start(buffer: CharSequence, startOffset: Int, endOffset: Int, initialState: Int) {
        this.buffer = buffer
        this.endOffset = endOffset
        this.tokenStart = startOffset
        this.tokenEnd = startOffset
        advance()
    }

    override fun getState(): Int = 0
    override fun getTokenType(): IElementType? = tokenType
    override fun getTokenStart(): Int = tokenStart
    override fun getTokenEnd(): Int = tokenEnd
    override fun getBufferSequence(): CharSequence = buffer
    override fun getBufferEnd(): Int = endOffset

    override fun advance() {
        if (tokenEnd >= endOffset) {
            tokenType = null
            return
        }

        var i = tokenEnd

        if (buffer[i].isWhitespace()) {
            tokenStart = i
            while (i < endOffset && buffer[i].isWhitespace()) i++
            tokenEnd = i
            tokenType = TokenType.WHITE_SPACE
            return
        }

        tokenStart = i
        val c = buffer[i]

        if (c == '/' && i + 1 < endOffset && buffer[i + 1] == '/') {
            i += 2
            while (i < endOffset && buffer[i] != '\n') i++
            tokenEnd = i
            tokenType = HyleTokenTypes.COMMENT
            return
        }

        if (c == '#') {
            i++
            while (i < endOffset && isIdentPart(buffer[i])) i++
            tokenEnd = i
            tokenType = HyleTokenTypes.DIRECTIVE
            return
        }

        if (c == '"') {
            i++
            while (i < endOffset && buffer[i] != '"') {
                if (buffer[i] == '\\' && i + 1 < endOffset) i += 2 else i++
            }
            if (i < endOffset) i++
            tokenEnd = i
            tokenType = HyleTokenTypes.STRING
            return
        }

        if (c.isDigit() || (c == '.' && i + 1 < endOffset && buffer[i + 1].isDigit())) {
            i++
            while (i < endOffset && (buffer[i].isDigit() || buffer[i] == '.')) i++
            tokenEnd = i
            tokenType = HyleTokenTypes.NUMBER
            return
        }

        if (isIdentStart(c)) {
            i++
            while (i < endOffset && isIdentPart(buffer[i])) i++
            val text = buffer.subSequence(tokenStart, i).toString()
            tokenEnd = i
            tokenType = when {
                text in keywords -> HyleTokenTypes.KEYWORD
                text in types -> HyleTokenTypes.TYPE
                text in constants -> HyleTokenTypes.CONSTANT
                else -> HyleTokenTypes.IDENTIFIER
            }
            return
        }

        val two = if (i + 1 < endOffset) buffer.subSequence(i, i + 2).toString() else ""
        if (two in setOf("=>", "->", "==", "!=", "<=", ">=", "&&", "||")) {
            tokenEnd = i + 2
            tokenType = HyleTokenTypes.OPERATOR
            return
        }

        if (c in "{}()[]<>+-*/=:.;,") {
            tokenEnd = i + 1
            tokenType = HyleTokenTypes.OPERATOR
            return
        }

        tokenEnd = i + 1
        tokenType = TokenType.BAD_CHARACTER
    }

    private fun isIdentStart(c: Char): Boolean =
        c.isLetter() || c == '_'

    private fun isIdentPart(c: Char): Boolean =
        c.isLetterOrDigit() || c == '_'
}
