pipeline {
    agent {
        docker {
            label 'docker-agent'
            args '-v /var/run/docker.sock:/var/run/docker.sock'
        }
    }
    
    environment {
        GITHUB_CREDENTIALS = credentials('github')
        GHCR_CREDENTIALS = credentials('ghcr')

        IMAGE_NAME = 'ghcr.io/dimigomeal/dimigomeal-api'
        IMAGE_VERSION = 'latest'
    }
    
    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }
        
        stage('Build and Push Docker Image') {
            steps {
                script {
                    docker.image('docker:stable').inside('-v /var/run/docker.sock:/var/run/docker.sock') {
                        sh """
                            docker build -t ${IMAGE_NAME}:${IMAGE_TAG} .
                            echo ${GHCR_CREDENTIALS_PSW} | docker login ghcr.io -u ${GHCR_CREDENTIALS_USR} --password-stdin
                            docker push ${IMAGE_NAME}:${IMAGE_TAG}
                            docker tag ${IMAGE_NAME}:${IMAGE_TAG} ${IMAGE_NAME}:latest
                            docker push ${IMAGE_NAME}:latest
                            docker logout
                        """
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