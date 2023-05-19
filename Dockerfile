FROM docker.io/library/ubuntu:22.04

# Copy binary to image 
COPY target/release/node-template /usr/local/bin

# Create user
RUN useradd -m -u 1000 -U -s /bin/sh -d /laos laos 


# Set up directories and permissions
RUN mkdir -p /data /laos/.local/share && \
    chown -R laos:laos /data /laos/.local/share && \
    ln -s /data /laos/.local/share/laos 

# Check if executable works in this container
RUN su laos -c '/usr/local/bin/node-template --version'

# Clean up unnecessary directories
RUN rm -rf /usr/bin /usr/sbin

# Switch to user laos
USER laos

# Expose necessary ports
EXPOSE 30333 9933 9944 9615

# Define volumes
VOLUME ["/data"]

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/node-template"]