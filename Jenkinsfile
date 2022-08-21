pipeline {
    agent {
        docker {
            image 'rust:1.63-slim'
        }
    }
    
    stages {
        stage('Build') {
            steps {
                sh 'cargo build --release'
            }
        }

        stage('Linting') {
            steps {
                sh 'cargo clippy --release --all'
            }
        }
    }
}