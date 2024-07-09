pipeline {
    agent any
    
    environment {
        GITHUB_CREDENTIALS = credentials('github')
        GHCR_CREDENTIALS = credentials('ghcr')

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
                    docker.build("ghcr.io/${IMAGE_NAME}:${IMAGE_VERSION}")
                }
            }
        }
        
        stage('Push to GHCR') {
            steps {
                script {
                    docker.withRegistry('https://ghcr.io', 'ghcr-credentials') {
                        docker.image("ghcr.io/${IMAGE_NAME}:${IMAGE_VERSION}").push()
                    }
                }
            }
        }
        
        // stage('Deploy') {
        //     steps {
        //         sh './deploy.sh'
        //     }
        // }
    }
}