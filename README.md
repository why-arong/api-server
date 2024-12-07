# Rust CRUD API Server
`Actix-web`과 `SQLx`를 사용하여 구현한 RESTful API 서버입니다. 
서버는 `Health Check` 엔드포인트와 `CRUD` 기능을 통해 **사용자 관리**를 제공합니다.

Docker Hub에 배포된 이미지를 사용하여 바로 서버를 실행할 수 있습니다.
```bash
docker pull pilmokim/love-rust
```
혹은 `git clone` 을 하시고 직접 빌드하셔도 됩니다.

이 프로젝트는 Kubernetes 환경에서 배포를 목표로 설계되었습니다.
다음의 프로젝트를 참고하면 aws 클라우드 상의 인프라를 구성하고 이 프로젝트의 서버를 배포할 수 있습니다.

##### https://github.com/why-arong/helm-deploy

## Prerequisites
- Rust: Rust 설치
- PostgreSQL: PostgreSQL 설치
- SQLx CLI: 데이터베이스 마이그레이션을 위해 필요:
    ```bash
    cargo install sqlx-cli --features postgres
    ```

## Setup
1. Clone the repository
   ```zsh
    git clone https://github.com/why-arong/api-server.git
    cd api-server
    ```
2. Configure environment variables
    ```bash
    DATABASE_URL=postgres://<username>:<password>@localhost/<database_name>
    HOST=0.0.0.0
    PORT=8080
    ```
3. Run database migrations
    ```shell
    sqlx migrate run
    ```

## Endpoints
### Health Check
- GET `/health`
  - 설명: 서버가 정상적으로 작동하는지 확인.
  - 응답: "OK"

### 사용자 관리

####  1. 사용자 생성
   - `POST` /users
        - 요청 본문 (JSON):
   ```json
   {
   "name": "Why Arong",
   "email": "whyarong@example.com",
   "password": "securepassword123"
   }
   ```
   - 응답:
   ```json
   {
   "id": "uuid",
   "name": "Why Arong",
   "email": "whyarong@example.com",
   "password_hash": "hashed_password",
   "created_at": "timestamp",
   "updated_at": "timestamp"
   }
   ```

#### 2. 모든 사용자 조회
   - `GET` /users
   - 응답:
   ```json
   [
       {
       "id": "uuid",
       "name": "Why Arong",
       "email": "whyarong@example.com",
       "password_hash": "hashed_password",
       "created_at": "timestamp",
       "updated_at": "timestamp"
       }
   ]
   ```
### 3. 특정 사용자 조회
   - `GET` /users/{id}
   - 응답:
   ```json
   {
   "id": "uuid",
   "name": "Why Arong",
   "email": "whyarong@example.com",
   "password_hash": "hashed_password",
   "created_at": "timestamp",
   "updated_at": "timestamp"
   }
   ```
### 4. 사용자 정보 수정
   `PUT` /users/{id}
   -  요청 본문 (JSON):
   ```json
   {
   "name": "Why Arong Updated",
   "email": "updated.arong@example.com",
   "password": "newsecurepassword123"
   }
   ```
   - 응답:
   ```json
   {
   "id": "uuid",
   "name": "Why Arong Updated",
   "email": "updated.arong@example.com",
   "password_hash": "hashed_password",
   "created_at": "timestamp",
   "updated_at": "timestamp"
   }
   ```
### 5. 사용자 삭제
   - `DELETE` /users/{id}
   - 응답: "User deleted"
   
## 테스트
통합 테스트 실행,

```bash
cargo test
```
