# JsonWebLog

실시간 JSON 로그 모니터링 애플리케이션

## 기능

- **실시간 로그 모니터링**: JSONL 형태의 로그를 stdin으로 받아 웹 UI에서 실시간 표시
- **자동 포트 할당**: 요청한 포트가 사용 중이면 자동으로 다음 가용 포트 찾기
- **고급 필터링**: 레벨, 검색어, 로거, 모듈별 필터링
- **실시간 업데이트**: WebSocket을 통한 실시간 로그 스트리밍
- **성능 최적화**: 가상화된 테이블, 메모리 관리 (최대 100,000 엔트리)

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

지원하는 필드:
- `timestamp`: ISO 8601 형식 또는 Unix 타임스탬프
- `level`: TRACE, DEBUG, INFO, WARN, ERROR, FATAL
- `logger`: 로거 이름
- `message`: 로그 메시지
- `module`: 모듈 이름 (선택)
- `function`: 함수 이름 (선택)

예시:
```json
{"timestamp":"2025-07-31T10:30:45.123Z","level":"INFO","logger":"main","message":"Application started","module":"app.py","function":"main"}
```

## 웹 인터페이스

브라우저에서 `http://localhost:{port}`로 접속하여:
- 실시간 로그 확인
- 레벨별 필터링
- 검색어로 필터링
- 로거/모듈별 필터링
- WebSocket 연결 상태 확인

## 포트 충돌 테스트

여러 인스턴스를 동시에 실행하여 포트 자동 할당 테스트:

```bash
# Terminal 1
cargo run --release 3000

# Terminal 2 (자동으로 3001 포트 할당됨)
cargo run --release 3000

# Terminal 3 (자동으로 3002 포트 할당됨)
cargo run --release 3000
```