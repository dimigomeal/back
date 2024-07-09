pipeline {
    agent any
    
    environment {
        IMAGE_NAME = 'dimigomeal/dimigomeal-api'
        IMAGE_VERSION = 'latest'
    }
    
    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }
        
        stage('Build Docker Image') {
            steps {
                script {
                    docker.build("ghcr.io/${env.IMAGE_NAME}:${env.IMAGE_VERSION}")
                }
            }
        }
        
        stage('Push to GHCR') {
            steps {
                script {
                    docker.withRegistry('https://ghcr.io', 'ghcr') {
                        docker.image("ghcr.io/${env.IMAGE_NAME}:${env.IMAGE_VERSION}").push()
                    }
                }
            }
        }
    }
}