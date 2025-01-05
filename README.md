# 🦀 mini-rusaint
Rust 기반 u-saint 클라이언트인 [rusaint](https://github.com/EATSTEAK/rusaint)의 간단한 구현입니다.

## 구현할 기능 목록
- [x] SAP 이벤트 형식으로 인코딩 된 문자열을 디코딩하는 기능
- [x] SAP SSO 토큰으로 유세인트 세션을 생성하는 기능
- [x] SAP SSR Client 정보를 가져오는 기능
- [x] SAP EVENT를 생성하는 기능
- [x] 학기별 성적을 가져오는 기능
- [x] (학년, 학기)별 세부 성적을 가져오는 기능 
- [ ] 과목의 상세성적을 가져오는 기능

## 환경 변수
유세인트 세션을 생성하기 위해 유세인트 아이디(학번)와 비밀번호를 환경 변수로 추가해야합니다.

`Cargo.toml` 파일이 위치한 디렉토리 위치에 `.env` 파일을 생성하고 아래와 같이 환경 변수를 추가합니다.

```
USAINT_ID={유세인트 아이디(학번)}
USAINT_PASSWORD={유세인트 비밀번호}
```

## 참조
이 프로젝트는 아래의 코드를 포함하고 있습니다:
- [rusaint](https://github.com/EATSTEAK/rusaint) - Copyright (c) [2023] [Hyomin Koo <me@eatsteak.dev>], MIT License