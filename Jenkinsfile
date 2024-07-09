pipeline {
    agent any
    
    environment {
        CONTAINER_NAME = 'dimigomeal-api'
        IMAGE_NAME = 'dimigomeal/dimigomeal-api'
        IMAGE_VERSION = 'latest'
        IMAGE_URL = "ghcr.io/${env.IMAGE_NAME}:${env.IMAGE_VERSION}"
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
                    docker.build(env.IMAGE_URL)
                }
            }
        }
        
        stage('Push to GHCR') {
            steps {
                script {
                    docker.withRegistry('https://ghcr.io', 'ghcr') {
                        docker.image(env.IMAGE_URL).push()
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
                        ${env.IMAGE_URL}"
                    sh "docker start ${env.CONTAINER_NAME}"
                }
            }
        }
    }
}