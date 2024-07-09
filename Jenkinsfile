pipeline {
    agent any
    
    environment {
        CONTAINER_NAME = 'dimigomeal-api'
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

        stage('Remove Container') {
            steps {
                script {
                    sh "docker rm -f ${env.CONTAINER_NAME} || true"
                }
            }
        }

        stage('Deploy Container') {
            steps {
                script {
                    sh "docker create \
                        --name ${env.CONTAINER_NAME} \
                        --restart always \
                        --network proxy \
                        --volume /mnt/docker/services/dimigomeal/db.db3:/db.db3 \
                        --volume /mnt/docker/services/dimigomeal/ios-activity.p8:/ios-activity.p8:ro \
                        ${env.IMAGE_NAME}:${env.IMAGE_VERSION}"
                    sh "docker start ${env.CONTAINER_NAME}"
                }
            }
        }
    }
}