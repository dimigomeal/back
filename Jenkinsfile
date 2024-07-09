pipeline {
    agent any
    
    environment {
        GIT_COMMIT_SHORT = sh(script: "git rev-parse --short HEAD", returnStdout: true).trim()
        CONTAINER_NAME = 'dimigomeal-api'
        REGISTRY_URL = 'ghcr.io'

        IMAGE_NAME = 'dimigomeal/dimigomeal-api'
        IMAGE_TAG = "${env.GIT_COMMIT_SHORT}-${env.BUILD_ID}"
        IMAGE_URL = "${env.REGISTRY_URL}/${env.IMAGE_NAME}:${env.IMAGE_TAG}"
    }
    
    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }
        
        stage('Build Image') {
            steps {
                script {
                    docker.build(env.IMAGE_URL)
                }
            }
        }
        
        stage('Push Image') {
            steps {
                script {
                    docker.withRegistry('https://ghcr.io', 'ghcr') {
                        docker.image(env.IMAGE_URL).push()
                        docker.image(env.IMAGE_URL).push("latest")
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