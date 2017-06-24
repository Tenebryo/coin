import tensorflow as tf
import numpy as np

# NOTE: do not forget to name items that will be used by the bot to compute
# scores

learning_rate = 0.01
training_epochs = 101
batch_size = 128
display_step = 1

with tf.name_scope('value') as scope:
    
    #The input - 4 64-bit integers representing the black and white pieces, and
    #black and white mobility, respectively
    v_in = tf.placeholder(tf.float32, [None,256], name='input')
    
    #the desired output - the expected score of the given position. 
    v_score = tf.placeholder(tf.float32, [None, 1])
    
    #Hidden layer variables:
    v_w0 = tf.Variable(tf.random_normal([256,256]))
    v_w1 = tf.Variable(tf.random_normal([256,256]))
    v_w2 = tf.Variable(tf.random_normal([256,1]))
    v_b0 = tf.Variable(tf.random_normal([256]))
    v_b1 = tf.Variable(tf.random_normal([256]))
    v_b2 = tf.Variable(tf.random_normal([1]))
    
    #construct neural net
    v_pred = tf.tanh(
        v_b2 + tf.matmul(tf.tanh(
            v_b1 + tf.matmul(tf.tanh(
                v_b0 + tf.matmul(v_in, v_w0)
            ), v_w1)
        ), v_w2)
    )
    
    #the actual output node is an int, scaled by 1024 to get better resolution
    v_out = tf.cast(tf.round(v_pred * 1000), tf.int32, name='output')
    
    #optimizer target:
    v_cost = tf.reduce_mean(tf.reduce_sum(tf.pow(v_pred - v_score, 2), reduction_indices=1))
    v_opt  = tf.train.GradientDescentOptimizer(learning_rate).minimize(v_cost)
    
    
with tf.name_scope('policy') as scope:
    # tf Graph Input
    x = tf.placeholder(tf.float32, [None, 8, 8, 4], name='input')
    y = tf.placeholder(tf.float32, [None, 8, 8, 1])

    # Set model weights
    #first pass filters
    F0a = tf.Variable(tf.random_normal([1,8,4,16]))
    F0b = tf.Variable(tf.random_normal([8,1,4,16]))
    b0 = tf.Variable(tf.random_normal([16,8,8]))

    #Level 1 filters
    F1a = tf.Variable(tf.random_normal([1,8,16,16]))
    F1b = tf.Variable(tf.random_normal([8,1,16,16]))
    b1 = tf.Variable(tf.random_normal([16,8,8]))

    #Level 2 filters
    F2a = tf.Variable(tf.random_normal([1,8,16,16]))
    F2b = tf.Variable(tf.random_normal([8,1,16,16]))
    b2 = tf.Variable(tf.random_normal([16,8,8]))

    #Level 3 filters
    F3a = tf.Variable(tf.random_normal([1,8,16,16]))
    F3b = tf.Variable(tf.random_normal([8,1,16,16]))
    b3 = tf.Variable(tf.random_normal([16,8,8]))

    #Level 3 filters
    F4a = tf.Variable(tf.random_normal([1,8,16,16]))
    F4b = tf.Variable(tf.random_normal([8,1,16,16]))
    b4 = tf.Variable(tf.random_normal([16,8,8]))

    #Level 3 filters
    F5a = tf.Variable(tf.random_normal([1,8,16,16]))
    F5b = tf.Variable(tf.random_normal([8,1,16,16]))
    b5 = tf.Variable(tf.random_normal([16,8,8]))

    #Level 3 filters
    F6a = tf.Variable(tf.random_normal([1,8,16,16]))
    F6b = tf.Variable(tf.random_normal([8,1,16,16]))
    b6 = tf.Variable(tf.random_normal([16,8,8]))

    #Level 3 filters
    F7a = tf.Variable(tf.random_normal([1,8,16,16]))
    F7b = tf.Variable(tf.random_normal([8,1,16,16]))
    b7 = tf.Variable(tf.random_normal([16,8,8]))

    #Level 3 filters
    F8a = tf.Variable(tf.random_normal([1,8,16,16]))
    F8b = tf.Variable(tf.random_normal([8,1,16,16]))
    b8 = tf.Variable(tf.random_normal([16,8,8]))

    #Consolidation filters
    F = tf.Variable(tf.random_normal([1,1,16,1]))
    b = tf.Variable(tf.random_normal([8,8,1]))


    # Construct model
    L0a = tf.transpose(tf.nn.conv2d(x, F0a, [1,1,1,1], 'VALID'),[0,3,1,2])
    L0b = tf.transpose(tf.nn.conv2d(x, F0b, [1,1,1,1], 'VALID'),[0,3,1,2])
    L0= tf.transpose(tf.tanh(tf.matmul(L0a, L0b) + b0), [0,2,3,1])

    L1a = tf.transpose(tf.nn.conv2d(L0, F1a, [1,1,1,1], 'VALID'),[0,3,1,2])
    L1b = tf.transpose(tf.nn.conv2d(L0, F1b, [1,1,1,1], 'VALID'),[0,3,1,2])
    L1= tf.transpose(tf.tanh(tf.matmul(L1a, L1b) + b1), [0,2,3,1])

    L2a = tf.transpose(tf.nn.conv2d(L1, F2a, [1,1,1,1], 'VALID'),[0,3,1,2])
    L2b = tf.transpose(tf.nn.conv2d(L1, F2b, [1,1,1,1], 'VALID'),[0,3,1,2])
    L2= tf.transpose(tf.tanh(tf.matmul(L2a, L2b) + b2), [0,2,3,1])

    L3a = tf.transpose(tf.nn.conv2d(L2, F3a, [1,1,1,1], 'VALID'),[0,3,1,2])
    L3b = tf.transpose(tf.nn.conv2d(L2, F3b, [1,1,1,1], 'VALID'),[0,3,1,2])
    L3= tf.transpose(tf.tanh(tf.matmul(L3a, L3b) + b3), [0,2,3,1])

    L4a = tf.transpose(tf.nn.conv2d(L3, F4a, [1,1,1,1], 'VALID'),[0,3,1,2])
    L4b = tf.transpose(tf.nn.conv2d(L3, F4b, [1,1,1,1], 'VALID'),[0,3,1,2])
    L4= tf.transpose(tf.tanh(tf.matmul(L4a, L4b) + b4), [0,2,3,1])

    L5a = tf.transpose(tf.nn.conv2d(L4, F5a, [1,1,1,1], 'VALID'),[0,3,1,2])
    L5b = tf.transpose(tf.nn.conv2d(L4, F5b, [1,1,1,1], 'VALID'),[0,3,1,2])
    L5= tf.transpose(tf.tanh(tf.matmul(L5a, L5b) + b5), [0,2,3,1])

    L6a = tf.transpose(tf.nn.conv2d(L5, F6a, [1,1,1,1], 'VALID'),[0,3,1,2])
    L6b = tf.transpose(tf.nn.conv2d(L5, F6b, [1,1,1,1], 'VALID'),[0,3,1,2])
    L6= tf.transpose(tf.tanh(tf.matmul(L6a, L6b) + b6), [0,2,3,1])

    L7a = tf.transpose(tf.nn.conv2d(L6, F7a, [1,1,1,1], 'VALID'),[0,3,1,2])
    L7b = tf.transpose(tf.nn.conv2d(L6, F7b, [1,1,1,1], 'VALID'),[0,3,1,2])
    L7= tf.transpose(tf.tanh(tf.matmul(L7a, L7b) + b7), [0,2,3,1])

    L8a = tf.transpose(tf.nn.conv2d(L7, F8a, [1,1,1,1], 'VALID'),[0,3,1,2])
    L8b = tf.transpose(tf.nn.conv2d(L7, F8b, [1,1,1,1], 'VALID'),[0,3,1,2])
    L8= tf.transpose(tf.tanh(tf.matmul(L8a, L8b) + b8), [0,2,3,1])

    pred = tf.nn.sigmoid(tf.nn.conv2d(L8, F, [1,1,1,1], 'VALID') + b, name='output')

    # Minimize error using cross entropy
    # TODO: Modify this maybe? model seems to produce all 1s
    cost = tf.reduce_mean(tf.softmax_cross_entropy_with_logits(
        labels = tf.reshape(y, [None, 64]), 
        logits = tf.log(tf.reshape(pred, [None, 64])), 
    ))
    # Gradient Descent
    optimizer = tf.train.GradientDescentOptimizer(learning_rate).minimize(cost)



# Initializing the variables
init = tf.global_variables_initializer()

saver = tf.train.Saver(tf.global_variables())


with tf.Session() as session:
    session.run(init)

    

    pass
