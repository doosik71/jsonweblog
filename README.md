# JsonWebLog

실시간 JSON 로그 모니터링 애플리케이션

## 기능

- **실시간 로그 모니터링**: JSONL 형태의 로그를 stdin으로 받아 웹 UI에서 실시간 표시
- **고성능 웹 UI**: 대용량 로그(수만 건 이상)도 성능 저하 없이 부드럽게 탐색할 수 있도록 **가상 스크롤(Virtual Scrolling)** 기술 적용
- **동적 스키마 및 커스텀 컬럼**: 첫 번째 로그의 구조를 분석하여 동적으로 테이블 컬럼을 생성합니다. 사용자는 UI를 통해 각 컬럼의 너비, 순서, 가시성을 자유롭게 커스터마이징하고 설정을 저장할 수 있습니다.
- **강력한 필터링**: 레벨, 검색어, 로거, 모듈별 필터링
- **UI 테마**: 다크, 라이트, Solarized Dark, Monokai Pro 등 다양한 테마를 선택하고 저장할 수 있습니다.
- **컬럼 순서 변경**: 웹 UI에서 컬럼 헤더를 드래그 앤 드롭하여 컬럼 순서를 변경할 수 있습니다.
- **로그 레벨별 시각화**: 로그 레벨(TRACE, DEBUG, INFO, WARN, ERROR, FATAL)에 따라 로그 메시지의 텍스트 색상과 배경색이 다르게 표시되어 중요도를 한눈에 파악할 수 있습니다.
- **포트 자동 할당**: 요청한 포트가 사용 중이면 자동으로 다음 가용 포트 찾기
- **단일 바이너리 실행**: 웹 UI(HTML, CSS, JS)를 실행 파일 내에 모두 포함시켜, 별도의 정적 파일 없이 단일 바이너리만으로 모든 기능을 제공합니다.

## 작동 방식

1. **서버 시작**: `main.rs`에서 `WebServer`를 초기화하고, 명령줄 인자로 받은 포트(기본 3000) 또는 사용 가능한 포트에서 웹 서버를 실행합니다.
2. **표준 입력 처리**: `server.rs`의 `stdin_parser_task`가 백그라운드에서 비동기적으로 표준 입력을 감시합니다.
3. **로그 파싱**: `parser.rs`의 `JsonLogParser`가 stdin으로 들어온 각 줄을 JSON으로 파싱하여 `LogEntry` 객체로 변환합니다.
4. **상태 관리 및 브로드캐스트**: 파싱된 로그는 `server.rs`의 `AppState`에 저장되며, 동시에 WebSocket 채널(`log_tx`)을 통해 연결된 모든 웹 클라이언트에 실시간으로 전송됩니다.
5. **웹 UI 렌더링**: 사용자가 브라우저로 접속하면 `ui/static_files.rs`에 내장된 HTML, CSS, JS 파일을 받아 웹 UI가 렌더링됩니다.
6. **실시간 업데이트**: 웹 UI의 JavaScript(`app.js`)는 WebSocket 연결을 통해 새로운 로그를 수신하고, 가상 스크롤 뷰에 동적으로 로그를 추가합니다.

## 소스 코드 구조

- `src/main.rs`: 애플리케이션 진입점. 서버를 설정하고 실행합니다.
- `src/server.rs`: `axum` 웹 서버의 핵심 로직. 라우팅, WebSocket 처리, 상태 관리를 담당합니다.
- `src/parser.rs`: 표준 입력으로부터 JSONL 로그를 파싱하는 로직을 구현합니다.
- `src/log_entry.rs`: `LogEntry`, `LogLevel` 등 로그 데이터의 핵심 자료 구조를 정의합니다.
- `src/filter.rs`: 로그 필터링 로직을 담당합니다.
- `src/schema.rs`: 로그 데이터의 동적 스키마와 사용자가 설정한 테이블 컬럼 구성을 관리합니다.
- `src/ui/static_files.rs`: 웹 UI를 구성하는 HTML, CSS, JavaScript 파일을 Rust 상수로 포함하고 서빙하는 역할을 합니다.

## 빌드

```bash
cargo build --release
```

## 사용법

### 기본 실행 (포트 3000)

```bash
cargo run --release
```

### 특정 포트 지정

```bash
cargo run --release 8080
```

### 로그 파일 스트리밍

```bash
# 정적 파일
cat logs.jsonl | cargo run --release

# 실시간 스트리밍
tail -f /path/to/logs.jsonl | cargo run --release 8080
```

## 포트 자동 할당

프로그램은 다음과 같이 포트를 할당합니다:

1. 사용자가 지정한 포트(또는 기본 3000번) 시도
2. 해당 포트가 사용 중이면 1씩 증가하며 가용한 포트 찾기
3. 최대 100회 시도 후 실패시 오류 반환
4. 콘솔에 실제 사용되는 포트 번호 출력

## 테스트

### 자동 테스트 스크립트

```bash
# 기본 포트(3000)로 테스트
testme.bat

# 특정 포트로 테스트
testme.bat 8080
```

### 수동 테스트

```bash
# 1. 서버 시작
cargo run --release 3000

# 2. 다른 터미널에서 로그 전송
echo '{"timestamp":"2025-07-31T12:00:00Z","level":"INFO","logger":"test","message":"Hello World"}' | cargo run --release 3000
```

## JSONL 로그 형식

`jsonweblog`는 표준 입력으로 들어오는 JSONL(JSON Lines) 형식의 로그를 파싱합니다.
첫 번째 로그 엔트리의 구조를 분석하여 동적으로 테이블 컬럼을 생성하며, `timestamp`, `level`, `message` 필드는 특별히 인식하여 처리합니다. 따라서 특정 필드에 얽매이지 않고 다양한 형태의 JSON 로그를 시각화할 수 있습니다.

**자동 인식 및 파싱되는 특별 필드:**

- `timestamp`: ISO 8601 형식 또는 Unix 타임스탬프 (다양한 형식 지원)
- `level`: TRACE, DEBUG, INFO, WARN, ERROR, FATAL 등 표준 로그 레벨 (대소문자 구분 없음)
- `message`: 로그 메시지 본문

**예시:**

```json
{"timestamp":"2025-07-31T10:30:45.123Z","level":"INFO","logger":"main","message":"Application started","module":"app.py","function":"main","custom_field":"value"}
```

## 웹 인터페이스

브라우저에서 `http://localhost:{port}`로 접속하여:

- 실시간 로그 확인
- 레벨별 필터링
- 검색어로 필터링
- 로거/모듈별 필터링
- UI 테마 변경
- 컬럼 순서 변경
- 로그 레벨에 따른 색상 및 배경색 시각화
- WebSocket 연결 상태 확인